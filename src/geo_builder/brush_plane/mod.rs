use std::cmp::Ordering;

mod geometry;
mod normals;
mod tangents;
mod uvs;
mod vertices;

use crate::map::quake::BrushPlane;
use crate::map::quake::Entity;
use crate::TextureInfo;
use crate::Vector3;
pub use geometry::Geometry;

use crate::Vertex;

pub fn build(
    TextureInfo(texture_info): &TextureInfo,
    entity: &Entity,
    planes: &[BrushPlane],
    plane: &BrushPlane,
) -> Geometry {
    let texture_info = texture_info.get(&plane.texture);

    let plane_vertices: Vec<Vertex> = planes
        .iter()
        .flat_map(|p1| {
            planes
                .iter()
                .flat_map(move |p2| build_plane_vertex(texture_info, entity, planes, plane, p1, p2))
        })
        .collect();

    let unique_vertices: Vec<Vertex> = plane_vertices
        .iter()
        .enumerate()
        .flat_map(|(i, vertex)| {
            // Find unique vertices and aggregate phong normals
            match plane_vertices
                .iter()
                .skip(i + 1)
                .find(|comp| comp.vertex == vertex.vertex)
            {
                None => match entity.get_property("_phong") {
                    Some("1") => {
                        let mut vertex = plane_vertices.iter().skip(i + 1).fold(
                            vertex.clone(),
                            |mut acc, next| {
                                if next.vertex == acc.vertex {
                                    acc.normal = next.normal;
                                }
                                acc
                            },
                        );
                        vertex.normal = vertex.normal.normalize();
                        Some(vertex)
                    }
                    _ => Some(vertex.clone()),
                },
                _ => None,
            }
        })
        .collect();

    let center: Vector3 = unique_vertices
        .iter()
        .fold(Vector3::new(0.0, 0.0, 0.0), |acc, next| acc + next.vertex)
        / unique_vertices.len().max(1) as f32;

    let mut local_vertices = unique_vertices;
    for vertex in local_vertices.iter_mut() {
        vertex.vertex -= center;
    }

    let u_axis = (plane.v1 - plane.v0).normalize();
    let v_axis = plane.normal().cross(u_axis);

    let mut wound_vertices = local_vertices;
    wound_vertices.sort_by(|a, b| {
        let vert_a = a.vertex;
        let vert_b = b.vertex;

        let lhs_pu = vert_a.dot(u_axis);
        let lhs_pv = vert_a.dot(v_axis);

        let rhs_pu = vert_b.dot(u_axis);
        let rhs_pv = vert_b.dot(v_axis);

        let lhs_angle = lhs_pv.atan2(lhs_pu);
        let rhs_angle = rhs_pv.atan2(rhs_pu);

        rhs_angle.partial_cmp(&lhs_angle).unwrap_or(Ordering::Equal)
    });

    let mut world_vertices = wound_vertices;
    for vertex in world_vertices.iter_mut() {
        vertex.vertex += center;
    }

    let indices: Vec<usize> = if world_vertices.len() < 3 {
        Vec::new()
    } else {
        (0..world_vertices.len() - 2)
            .flat_map(|i| vec![0, i + 1, i + 2])
            .collect()
    };

    let texture = match texture_info {
        Some(_texture) => Some(plane.texture.clone()),
        None => None,
    };

    Geometry::new(center, world_vertices, indices, texture)
}

fn build_plane_vertex(
    texture_info: Option<&crate::Texture>,
    entity: &Entity,
    planes: &[BrushPlane],
    plane: &BrushPlane,
    p1: &BrushPlane,
    p2: &BrushPlane,
) -> Option<Vertex> {
    if let Some(vertex) = vertices::intersect_brush_planes(plane, p1, p2) {
        if vertices::vertex_in_hull(vertex, planes) {
            let normal = normals::vertex_normal(entity, plane, p1, p2);
            let tangent = tangents::vertex_tangent(plane);

            let uv = match &texture_info {
                Some(texture) => Some(uvs::vertex_uv(vertex, plane, texture)),
                None => None,
            };

            let color = match &plane.extra {
                crate::map::quake::ExtraData::Daikatana(_surface_data, color) => Some(*color),
                _ => None,
            };

            return Some(Vertex::new(vertex, normal, tangent, uv, color));
        }
    }

    None
}
