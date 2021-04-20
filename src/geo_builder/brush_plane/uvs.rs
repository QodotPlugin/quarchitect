use crate::map::quake::BrushPlane;
use crate::map::UV;
use crate::Texture;
use crate::Vector2;
use crate::Vector3;

pub fn vertex_uv(vertex: Vector3, brush_plane: &BrushPlane, texture: &Texture) -> Vector2 {
    match &brush_plane.uv {
        UV::Quake(_uv) => standard_uv(vertex, brush_plane, texture),
        UV::Valve(_uv) => valve_uv(vertex, brush_plane, texture),
    }
}

fn standard_uv(vertex: Vector3, brush_plane: &BrushPlane, texture: &Texture) -> Vector2 {
    let up_vector: Vector3 = Vector3::new(0.0, 0.0, 1.0);
    let right_vector: Vector3 = Vector3::new(0.0, 1.0, 0.0);
    let forward_vector: Vector3 = Vector3::new(1.0, 0.0, 0.0);

    let du = brush_plane.normal().dot(up_vector).abs();
    let dr = brush_plane.normal().dot(right_vector).abs();
    let df = brush_plane.normal().dot(forward_vector).abs();

    let (x, y);
    if du >= dr && du >= df {
        x = vertex.x();
        y = -vertex.y();
    } else if dr >= du && dr >= df {
        x = vertex.x();
        y = -vertex.z();
    } else if df >= du && df >= dr {
        x = vertex.y();
        y = -vertex.z();
    }
    else {
        panic!("Zero-length normal");
    }


    let rot = crate::Mat2::from_angle(brush_plane.rotation.to_radians());

    let uv = rot * Vector2::new(x, y);

    let uv = uv / texture.size();
    let uv = uv / brush_plane.scale;

    let mut uv = uv;
    if let UV::Quake(standard_uv) = &brush_plane.uv {
        uv += Vector2::new(standard_uv.u, standard_uv.v) / texture.size();
    } else {
        panic!("Not a standard UV");
    }

    uv
}

fn valve_uv(vertex: Vector3, brush_plane: &BrushPlane, texture: &Texture) -> Vector2 {
    let mut uv;
    if let UV::Valve(valve_uv) = &brush_plane.uv {
        uv = Vector2::new(valve_uv.u_axis.dot(vertex), valve_uv.v_axis.dot(vertex));

        uv /= texture.size();
        uv /= brush_plane.scale;
        uv += Vector2::new(valve_uv.u_offset, valve_uv.v_offset) / texture.size();
    } else {
        panic!("Not a valve UV");
    }

    uv
}
