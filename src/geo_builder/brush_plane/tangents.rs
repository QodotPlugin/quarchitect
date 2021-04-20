use crate::map::quake::BrushPlane;
use crate::map::UV;
use crate::Vector3;

pub fn vertex_tangent(brush_plane: &BrushPlane) -> (Vector3, f32) {
    match &brush_plane.uv {
        UV::Quake(_uv) => standard_tangent(brush_plane),
        UV::Valve(_uv) => valve_tangent(brush_plane),
    }
}

fn standard_tangent(brush_plane: &BrushPlane) -> (Vector3, f32) {
    let up_vector: Vector3 = Vector3::new(0.0, 0.0, 1.0);
    let right_vector: Vector3 = Vector3::new(0.0, 1.0, 0.0);
    let forward_vector: Vector3 = Vector3::new(1.0, 0.0, 0.0);

    let du = brush_plane.normal().dot(up_vector);
    let dr = brush_plane.normal().dot(right_vector);
    let df = brush_plane.normal().dot(forward_vector);

    let du_abs = du.abs();
    let dr_abs = dr.abs();
    let df_abs = df.abs();

    let mut tangent: Option<(Vector3, f32)> = None;

    if du_abs >= dr_abs && du_abs >= df_abs {
        tangent = Some((forward_vector, du.signum()));
    } else if dr_abs >= du_abs && dr_abs >= df_abs {
        tangent = Some((forward_vector, -dr.signum()));
    } else if df_abs >= du_abs && df_abs >= dr_abs {
        tangent = Some((right_vector, df.signum()));
    }

    match tangent {
        Some((v, s)) => (
            crate::Quat::from_axis_angle(brush_plane.normal(), -brush_plane.rotation.to_radians() * s) * v,
            s * brush_plane.scale.y().signum(),
        ),
        None => panic!("Failed to generate tangent"),
    }
}

fn valve_tangent(brush_plane: &BrushPlane) -> (Vector3, f32) {
    if let UV::Valve(uv) = &brush_plane.uv {
        let v_sign = -brush_plane
            .normal()
            .cross(uv.u_axis)
            .dot(uv.v_axis)
            .signum();
        (uv.u_axis, v_sign)
    } else {
        panic!("Not a valve UV");
    }
}
