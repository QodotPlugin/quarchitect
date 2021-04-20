use nom::{character::complete::anychar, number::complete::le_u32, IResult};

#[derive(Debug)]
pub struct Header {
    pub magic: [char; 4],
    pub num_entries: u32,
    pub dir_offset: u32,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            magic: ['\0'; 4],
            num_entries: 0,
            dir_offset: 0,
        }
    }
}

fn wad_magic(i: &[u8]) -> IResult<&[u8], Vec<char>> {
    nom::multi::count(anychar, 4)(i)
}

pub fn parser(i: &[u8]) -> IResult<&[u8], Header> {
    let (i, o) = nom::sequence::tuple((wad_magic, le_u32, le_u32))(i)?;

    let (magic, num_entries, dir_offset) = o;

    let mut magic_arr = ['\0'; 4];
    magic_arr.copy_from_slice(&magic[..]);

    Ok((
        i,
        Header {
            magic: magic_arr,
            num_entries,
            dir_offset,
        },
    ))
}
