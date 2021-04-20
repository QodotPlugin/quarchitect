use nom::{number::complete::le_u8, IResult};

#[derive(Debug)]
pub struct MipLevel(pub Vec<u8>);

impl MipLevel {
    pub fn new(colors: Vec<u8>) -> MipLevel {
        MipLevel(colors)
    }
}

pub fn parser<'a>(texture_size: usize) -> impl Fn(&[u8]) -> IResult<&[u8], MipLevel> + 'a {
    move |i: &[u8]| {
        let (i, o) = nom::multi::count(le_u8, texture_size)(i)?;
        Ok((i, MipLevel::new(o)))
    }
}
