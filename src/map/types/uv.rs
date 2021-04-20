use crate::Vector3;

#[derive(PartialEq, Debug)]
pub enum UV {
    Quake(QuakeUV),
    Valve(ValveUV),
}

impl UV {
    pub fn standard(u: f32, v: f32) -> UV {
        UV::Quake(QuakeUV::new(u, v))
    }

    pub fn valve(u_axis: Vector3, u_offset: f32, v_axis: Vector3, v_offset: f32) -> UV {
        UV::Valve(ValveUV::new(u_axis, u_offset, v_axis, v_offset))
    }
}

#[derive(PartialEq, Debug)]
pub struct QuakeUV {
    pub u: f32,
    pub v: f32,
}

impl QuakeUV {
    fn new(u: f32, v: f32) -> QuakeUV {
        QuakeUV { u, v }
    }
}

#[derive(PartialEq, Debug)]
pub struct ValveUV {
    pub u_axis: Vector3,
    pub u_offset: f32,
    pub v_axis: Vector3,
    pub v_offset: f32,
}

impl ValveUV {
    fn new(u_axis: Vector3, u_offset: f32, v_axis: Vector3, v_offset: f32) -> ValveUV {
        ValveUV {
            u_axis,
            u_offset,
            v_axis,
            v_offset,
        }
    }
}