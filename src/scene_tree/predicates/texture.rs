use crate::TextureBlacklist;

pub fn unique<'a>(textures: &'a [String]) -> impl Fn(&(usize, String)) -> bool + 'a {
    move |(i, texture): &(usize, String)| {
        textures
            .iter()
            .skip(i + 1)
            .find(|comp| *comp == texture)
            .is_none()
    }
}

pub fn not_blacklisted<'a>(
    texture_blacklist: &'a TextureBlacklist,
) -> impl Fn(&(usize, String)) -> bool + 'a {
    move |(_, texture): &(usize, String)| !texture_blacklist.is_blacklisted_plane(texture)
}

pub fn unique_not_blacklisted<'a>(
    textures: &'a [String],
    texture_blacklist: &'a TextureBlacklist,
) -> impl Fn(&(usize, String)) -> bool + 'a {
    move |i: &(usize, String)| unique(textures)(i) && not_blacklisted(&texture_blacklist)(i)
}
