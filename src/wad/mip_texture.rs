use nom::{character::complete::anychar, number::complete::le_u32, IResult};

#[derive(Debug)]
pub struct MipTexture {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub offset1: u32,
    pub offset2: u32,
    pub offset4: u32,
    pub offset8: u32,
}

pub fn parser(i: &[u8]) -> IResult<&[u8], MipTexture> {
    let (i, o) = nom::sequence::tuple((
        nom::multi::count(anychar, 16),
        le_u32,
        le_u32,
        le_u32,
        le_u32,
        le_u32,
        le_u32,
    ))(i)?;

    let (name, width, height, offset1, offset2, offset4, offset8) = o;

    let name: String = name
        .into_iter()
        .take_while(|c| *c != char::from(0))
        .collect();

    let name = name.to_lowercase();

    Ok((
        i,
        MipTexture {
            name,
            width,
            height,
            offset1,
            offset2,
            offset4,
            offset8,
        },
    ))
}
