use nom::{
    character::complete::anychar,
    number::complete::{le_u16, le_u32, le_u8},
    IResult,
};

#[derive(Debug)]
pub struct Entry {
    pub offset: u32,
    pub dsize: u32,
    pub size: u32,
    pub entry_type: char,
    pub cmprs: u8,
    pub name: String,
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            offset: 0,
            dsize: 0,
            size: 0,
            entry_type: '\0',
            cmprs: 0,
            name: String::new(),
        }
    }
}

pub fn parser(i: &[u8]) -> IResult<&[u8], Entry> {
    let (i, o) = nom::sequence::tuple((
        le_u32,
        le_u32,
        le_u32,
        anychar,
        le_u8,
        le_u16,
        nom::multi::count(anychar, 16),
    ))(i)?;

    let (offset, dsize, size, entry_type, cmprs, _, name) = o;

    let name: String = name
        .into_iter()
        .take_while(|c| *c != char::from(0))
        .collect();

    Ok((
        i,
        Entry {
            offset,
            dsize,
            size,
            entry_type,
            cmprs,
            name,
        },
    ))
}
