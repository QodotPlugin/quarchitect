use crate::map::UV;
use crate::Color;
use crate::QuarchitectError;
use crate::Vector2;
use crate::Vector3;

// Game-specific extra data
#[derive(PartialEq, Debug)]
pub struct SurfaceData {
    pub surface_contents: i32,
    pub surface_flags: i32,
    pub surface_value: f32,
}

impl SurfaceData {
    fn new(surface_contents: i32, surface_flags: i32, surface_value: f32) -> SurfaceData {
        SurfaceData {
            surface_contents,
            surface_flags,
            surface_value,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ExtraData {
    None,
    Hexen2(f32),
    Quake2(SurfaceData),
    Daikatana(SurfaceData, Color),
}

impl ExtraData {
    pub fn quake_2(comps: Vec<&str>) -> Result<ExtraData, QuarchitectError> {
        let surface_contents: i32 = match comps[0].parse() {
            Ok(surface_contents) => surface_contents,
            Err(_err) => return Err(QuarchitectError("Failed to parse surface contents")),
        };

        let surface_flags: i32 = match comps[1].parse() {
            Ok(surface_flags) => surface_flags,
            Err(_err) => return Err(QuarchitectError("Failed to parse surface flags")),
        };

        let surface_value: f32 = match comps[2].parse() {
            Ok(surface_value) => surface_value,
            Err(_err) => return Err(QuarchitectError("Failed to parse surface value")),
        };

        Ok(ExtraData::Quake2(SurfaceData::new(
            surface_contents,
            surface_flags,
            surface_value,
        )))
    }

    pub fn daikatana(comps: Vec<&str>) -> Result<ExtraData, QuarchitectError> {
        let quake_2_data = ExtraData::quake_2(comps.clone())?;
        if let ExtraData::Quake2(surface_data) = quake_2_data {
            let r: f32 = match comps[3].parse() {
                Ok(r) => r,
                Err(_err) => return Err(QuarchitectError("Failed to parse R color component")),
            };

            let g: f32 = match comps[4].parse() {
                Ok(g) => g,
                Err(_err) => return Err(QuarchitectError("Failed to parse G color component")),
            };

            let b: f32 = match comps[5].parse() {
                Ok(b) => b,
                Err(_err) => return Err(QuarchitectError("Failed to parse B color component")),
            };

            Ok(ExtraData::Daikatana(surface_data, Color::new(r, g, b)))
        } else {
            panic!();
        }
    }
}

// Brush Plane
#[derive(PartialEq, Debug)]
pub struct BrushPlane {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
    pub texture: String,
    pub uv: UV,
    pub rotation: f32,
    pub scale: Vector2,
    pub extra: ExtraData,
}

impl BrushPlane {
    pub fn new(source: &str) -> Result<BrushPlane, QuarchitectError> {
        let mut comps = source.split_whitespace();

        let mut v0 = Vector3::new(0.0, 0.0, 0.0);
        let mut v1 = Vector3::new(0.0, 0.0, 0.0);
        let mut v2 = Vector3::new(0.0, 0.0, 0.0);

        for i in 0..3 {
            comps.next();
            let x: f32 = comps.next().unwrap().parse().unwrap();
            let y: f32 = comps.next().unwrap().parse().unwrap();
            let z: f32 = comps.next().unwrap().parse().unwrap();
            comps.next();

            let v = match i {
                0 => &mut v0,
                1 => &mut v1,
                2 => &mut v2,
                _ => panic!(),
            };

            v.set_x(x);
            v.set_y(y);
            v.set_z(z);
        }

        let texture = comps.next().unwrap().to_lowercase();

        let uv = match comps.next().unwrap().parse::<f32>() {
            Ok(u) => {
                let v: f32 = comps.next().unwrap().parse().unwrap();
                UV::standard(u, v)
            }
            Err(_err) => {
                let ux: f32 = comps.next().unwrap().parse().unwrap();
                let uy: f32 = comps.next().unwrap().parse().unwrap();
                let uz: f32 = comps.next().unwrap().parse().unwrap();
                let uo: f32 = comps.next().unwrap().parse().unwrap();

                comps.next();
                comps.next();

                let vx: f32 = comps.next().unwrap().parse().unwrap();
                let vy: f32 = comps.next().unwrap().parse().unwrap();
                let vz: f32 = comps.next().unwrap().parse().unwrap();
                let vo: f32 = comps.next().unwrap().parse().unwrap();
                
                comps.next();

                UV::valve(Vector3::new(ux, uy, uz), uo, Vector3::new(vx, vy, vz), vo)
            }
        };

        let rotation: f32 = comps.next().unwrap().parse().unwrap();

        let x: f32 = comps.next().unwrap().parse().unwrap();
        let y: f32 = comps.next().unwrap().parse().unwrap();
        let scale = Vector2::new(x, y);

        let extra_comps: Vec<&str> = comps.collect();
        let extra = match extra_comps.len() {
            0 => ExtraData::None,
            1 => ExtraData::Hexen2(extra_comps.first().unwrap().parse().unwrap()),
            3 => ExtraData::quake_2(extra_comps)?,
            6 => ExtraData::daikatana(extra_comps)?,
            _ => return Err(QuarchitectError("Unrecognized extra data")),
        };

        Ok(BrushPlane {
            v0,
            v1,
            v2,
            texture,
            uv,
            rotation,
            scale,
            extra,
        })
    }

    pub fn normal(&self) -> Vector3 {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        v0v2.cross(v0v1).normalize()
    }

    pub fn dist(&self) -> f32 {
        let n = self.normal();
        n.dot(self.v0)
    }
}
