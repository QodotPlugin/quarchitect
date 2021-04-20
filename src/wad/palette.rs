use nom::IResult;

use super::color::{self, Color};

#[derive(Copy, Clone)]
pub struct Palette(pub [Color; 256]);

pub fn parser(i: &[u8]) -> IResult<&[u8], Palette> {
    let (i, o) = nom::multi::count(color::parser, 256)(i)?;
    let mut color_arr = [Color(0, 0, 0); 256];
    color_arr.copy_from_slice(&o);
    Ok((i, Palette(color_arr)))
}

pub fn read_palette(palette_file: &str) -> Result<Palette, String> {
    let palette_buf = match std::fs::read(palette_file) {
        Ok(palette_buf) => palette_buf,
        Err(err) => return Err(format!("Failed to load palette: {:?}", err)),
    };

    let (_, palette) = match parser(&palette_buf) {
        Ok(palette) => palette,
        Err(err) => return Err(format!("Failed to parse palette: {:?}", err)),
    };

    Ok(palette)
}
