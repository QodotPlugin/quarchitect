use rayon::prelude::*;

mod color;
pub mod mip_level;
pub mod palette;

mod entry;
mod header;
mod mip_data;
mod mip_texture;

pub use color::Color;
pub use mip_data::{MipDataIndexed, MipDataRGB};
pub use mip_texture::MipTexture;

use entry::Entry;

use palette::Palette;
use std::{
    fs::File,
    io::{Read, Seek},
    time::Instant,
};

pub const HEADER_BYTES: usize = 12;
pub const ENTRY_BYTES: usize = 32;
pub const MIPTEX_BYTES: usize = 40;

#[derive(PartialEq, Eq)]
enum WadType {
    WAD2,
    WAD3,
}

pub struct TextureIndexed {
    pub mip_texture: MipTexture,
    pub mip_data: MipDataIndexed,
    pub palette: Option<Palette>,
}

impl TextureIndexed {
    pub fn new(
        mip_texture: MipTexture,
        mip_data: MipDataIndexed,
        palette: Option<Palette>,
    ) -> TextureIndexed {
        TextureIndexed {
            mip_texture,
            mip_data,
            palette,
        }
    }

    pub fn into_rgb(self, palette: Option<Palette>) -> Result<TextureRGB, String> {
        let palette = match self.palette {
            Some(texture_palette) => {
                if palette.is_some() {
                    println!("Skipping supplied palette: Not necessary for WAD3 RGB conversion");
                }
                texture_palette
            }
            None => match palette {
                Some(palette) => palette,
                None => return Err("WAD2 RGB conversion requires a palette".into()),
            },
        };

        Ok(TextureRGB::from_indexed(self, &palette))
    }
}

pub struct TextureRGB {
    pub mip_texture: MipTexture,
    pub mip_data: MipDataRGB,
}

impl TextureRGB {
    pub fn from_indexed(i: TextureIndexed, palette: &Palette) -> TextureRGB {
        let mip_texture = i.mip_texture;
        let mip_data = i.mip_data.into_rgb(palette);

        TextureRGB {
            mip_texture,
            mip_data,
        }
    }
}

pub fn read_textures(
    wad_file: &str,
    whitelist: Option<Vec<String>>,
    mip_levels: usize
) -> Result<Vec<TextureIndexed>, String> {
    assert!(mip_levels >= 1 && mip_levels <= 4);

    let mut file = match File::open(wad_file) {
        Ok(wad_file) => wad_file,
        Err(err) => return Err(format!("Error opening WAD file {:?}: {:?}", wad_file, err)),
    };

    let now = Instant::now();
    let header = read_header(&mut file)?;
    println!("Read header took {}", now.elapsed().as_millis());

    let wad_type = match header.magic {
        ['W', 'A', 'D', '2'] => WadType::WAD2,
        ['W', 'A', 'D', '3'] => WadType::WAD3,
        _ => panic!("Unexpected WAD magic: {:?}", header.magic),
    };

    let now = Instant::now();
    let directory: Vec<Entry> = read_directory(&mut file, &header, &whitelist)?;
    println!("Read directory took {}", now.elapsed().as_millis());

    let now = Instant::now();
    let mip_textures: Vec<MipTexture> = read_mip_textures(&mut file, &directory)?;
    println!("Read mip textures took {}", now.elapsed().as_millis());

    let now = Instant::now();
    let mip_data: Vec<(MipDataIndexed, Option<Palette>)> =
        read_mip_data(wad_file, wad_type, &directory, &mip_textures, mip_levels)?;
    println!("Read mip data took {}", now.elapsed().as_millis());

    let result: Vec<TextureIndexed> = mip_textures
        .into_iter()
        .zip(mip_data.into_iter())
        .map(|(mip_texture, (mip_data, palette))| {
            TextureIndexed::new(mip_texture, mip_data, palette)
        })
        .collect();

    Ok(result)
}

fn read_header(wad_file: &mut File) -> Result<header::Header, String> {
    let mut header_buf = [0u8; HEADER_BYTES];
    if let Err(err) = wad_file.read_exact(&mut header_buf) {
        return Err(format!("Error reading WAD header: {:?}", err));
    }

    match header::parser(&header_buf) {
        Ok((_, header)) => Ok(header),
        Err(err) => Err(format!("Error parsing WAD header: {:?}", err)),
    }
}

fn read_directory(
    wad_file: &mut File,
    header: &header::Header,
    whitelist: &Option<Vec<String>>,
) -> Result<Vec<Entry>, String> {
    let entries: Vec<Result<Option<Entry>, String>> = (0..header.num_entries)
        .map(|i| {
            let offset = header.dir_offset;
            let offset = offset + i * ENTRY_BYTES as u32;
            if let Err(err) = wad_file.seek(std::io::SeekFrom::Start(offset as u64)) {
                return Err(format!("Error seeking to directory: {:?}", err));
            }

            let mut entry_buf = [0u8; ENTRY_BYTES];
            if let Err(err) = wad_file.read_exact(&mut entry_buf) {
                return Err(format!("Error reading entry {}: {:?}", i, err));
            }

            let entry = match entry::parser(&entry_buf) {
                Ok((_i, entry)) => entry,
                Err(err) => return Err(format!("Error parsing entry: {:?}", err)),
            };

            match entry.entry_type {
                'C' | 'D' => (),
                _ => return Ok(None),
            };

            if match whitelist {
                None => true,
                Some(whitelist) => whitelist.iter().any(|texture| texture == &entry.name),
            } {
                Ok(Some(entry))
            } else {
                Ok(None)
            }
        })
        .collect();

    if let Some(err) = entries.iter().find_map(|res| match res {
        Err(err) => Some(err),
        _ => None,
    }) {
        return Err(err.clone());
    }

    let entries = entries
        .into_iter()
        .flat_map(|entry| match entry {
            Ok(Some(entry)) => Some(entry),
            _ => None,
        })
        .collect();

    Ok(entries)
}

fn read_mip_textures(wad_file: &mut File, directory: &[Entry]) -> Result<Vec<MipTexture>, String> {
    let mip_textures: Vec<Result<MipTexture, String>> = directory
        .iter()
        .map(move |entry| {
            if let Err(err) = wad_file.seek(std::io::SeekFrom::Start(entry.offset as u64)) {
                return Err(format!("Error seeking to miptex: {:?}", err));
            }

            let mut miptex_buf = [0u8; MIPTEX_BYTES];
            if let Err(err) = wad_file.read_exact(&mut miptex_buf) {
                return Err(format!("Error reading miptex: {:?}", err));
            }

            let miptex = match mip_texture::parser(&miptex_buf) {
                Ok((_, entry)) => entry,
                Err(err) => return Err(format!("Error parsing miptex: {:?}", err)),
            };

            Ok(miptex)
        })
        .collect();

    if let Some(err) = mip_textures.iter().find_map(|res| match res {
        Err(err) => Some(err),
        _ => None,
    }) {
        return Err(err.clone());
    }

    let mip_textures = mip_textures
        .into_iter()
        .flat_map(|mip_texture| match mip_texture {
            Ok(mip_texture) => Some(mip_texture),
            _ => None,
        })
        .collect();

    Ok(mip_textures)
}

fn read_mip_data(
    wad_file: &str,
    wad_type: WadType,
    directory: &[Entry],
    mip_textures: &[MipTexture],
    mip_levels: usize,
) -> Result<Vec<(MipDataIndexed, Option<Palette>)>, String> {
    assert!(mip_levels >= 1 && mip_levels <= 4);

    let mip_data: Vec<Result<(MipDataIndexed, Option<Palette>), String>> = directory
        .par_iter()
        .zip(mip_textures.par_iter())
        .map(|(entry, miptex)| {
            println!("Reading mipdata for entry: {:?}", entry);

            let mut wad_file = match File::open(wad_file) {
                Ok(wad_file) => wad_file,
                Err(err) => return Err(format!("Error opening WAD file: {:?}", err)),
            };

            if let Err(err) = wad_file.seek(std::io::SeekFrom::Start(
                (entry.offset + miptex.offset1) as u64,
            )) {
                return Err(format!("Error seeking to miptex: {:?}", err));
            }

            let size0 = (miptex.width * miptex.height) as usize;
            let size1 = size0 / 4;
            let size2 = size1 / 4;
            let size3 = size2 / 4;

            let sizes = [size0, size1, size2, size3];
            let sizes = Vec::from(&sizes[..mip_levels]);
            let total_size = sizes.iter().sum();

            let mut mipdata_buf = vec![0u8; total_size];

            if let Err(err) = wad_file.read_exact(&mut mipdata_buf) {
                return Err(format!(
                    "Error reading mipdata for texture {:?}: {:?}",
                    miptex, err
                ));
            }

            let mip0_buf = &mipdata_buf[..size0];

            let (_, mip0) = match mip_level::parser(size0)(mip0_buf) {
                Ok(mip_level) => mip_level,
                Err(err) => return Err(format!("Error parsing mip level 0: {:?}", err)),
            };

            let mip1 = if mip_levels > 1 {
                let mip1_buf = &mipdata_buf[size0..size1];
                let (_, mip1) = match mip_level::parser(size1)(mip1_buf) {
                    Ok(mip_level) => mip_level,
                    Err(err) => return Err(format!("Error parsing mip level 1: {:?}", err)),
                };
                Some(mip1)
            } else {
                None
            };

            let mip2 = if mip_levels > 2 {
                let mip2_buf = &mipdata_buf[size0 + size1..size2];
                let (_, mip2) = match mip_level::parser(size2)(mip2_buf) {
                    Ok(mip_level) => mip_level,
                    Err(err) => return Err(format!("Error parsing mip level 2: {:?}", err)),
                };
                Some(mip2)
            } else {
                None
            };

            let mip3 = if mip_levels > 3 {
                let mip3_buf = &mipdata_buf[size0 + size1 + size2..size3];
                let (_, mip3) = match mip_level::parser(size3)(mip3_buf) {
                    Ok(mip_level) => mip_level,
                    Err(err) => return Err(format!("Error parsing mip level 3: {:?}", err)),
                };
                Some(mip3)
            } else {
                None
            };

            let mipdata = MipDataIndexed {
                mip0,
                mip1,
                mip2,
                mip3,
            };

            let palette;
            match wad_type {
                WadType::WAD2 => palette = None,
                WadType::WAD3 => {
                    if let Err(err) = wad_file.seek(std::io::SeekFrom::Start(
                        (entry.offset + miptex.offset8 + (size3 as u32) + 2) as u64,
                    )) {
                        return Err(format!("Error seeking to texture palette: {:?}", err));
                    }

                    let mut palette_buf = vec![0u8; 256 * 3];

                    if let Err(err) = wad_file.read_exact(&mut palette_buf) {
                        return Err(format!("Error reading texture palette: {:?}", err));
                    }

                    let (_, texture_palette) = match palette::parser(&palette_buf) {
                        Ok(texture_palette) => texture_palette,
                        Err(err) => {
                            return Err(format!("Error parsing texture palette: {:?}", err))
                        }
                    };

                    palette = Some(texture_palette);
                }
            }

            Ok((mipdata, palette))
        })
        .collect();

    if let Some(err) = mip_data.iter().find_map(|res| match res {
        Err(err) => Some(err),
        _ => None,
    }) {
        return Err(err.clone());
    }

    let mip_data = mip_data
        .into_iter()
        .flat_map(|mip_data| match mip_data {
            Ok(mip_data) => Some(mip_data),
            _ => None,
        })
        .collect();

    Ok(mip_data)
}
