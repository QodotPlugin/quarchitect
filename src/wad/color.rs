use nom::IResult;
use nom::number::complete::le_u8;

#[derive(Debug, Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

pub fn parser(i: &[u8]) -> IResult<&[u8], Color> {
    let (i, (r, g, b)) = nom::sequence::tuple((le_u8, le_u8, le_u8))(i)?;
    Ok((i, Color(r, g, b)))
}
