use super::color::Color;
use super::palette::Palette;

use super::mip_level::MipLevel;

#[derive(Debug)]
pub struct MipDataIndexed {
    pub mip0: MipLevel,
    pub mip1: Option<MipLevel>,
    pub mip2: Option<MipLevel>,
    pub mip3: Option<MipLevel>,
}

impl MipDataIndexed {
    pub fn into_rgb(self, palette: &Palette) -> MipDataRGB {
        MipDataRGB::from_indexed(self, palette)
    }
}

#[derive(Debug)]
pub struct MipDataRGB {
    pub mip0: Vec<Color>,
    pub mip1: Option<Vec<Color>>,
    pub mip2: Option<Vec<Color>>,
    pub mip3: Option<Vec<Color>>,
}

impl MipDataRGB {
    pub fn from_indexed(mip_data: MipDataIndexed, Palette(palette): &Palette) -> Self {
        let to_color = |MipLevel(mip_level)| mip_level.iter().map(|idx| palette[*idx as usize]).collect();

        let mip0 = to_color(mip_data.mip0);
        let mip1 = mip_data.mip1.map(to_color);
        let mip2 = mip_data.mip2.map(to_color);
        let mip3 = mip_data.mip3.map(to_color);

        MipDataRGB {
            mip0,
            mip1,
            mip2,
            mip3,
        }
    }
}
