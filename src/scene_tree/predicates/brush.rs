use crate::TextureBlacklist;

pub fn not_blacklisted<'a>(
    texture_blacklist: &'a TextureBlacklist,
) -> impl Fn(&&crate::geo_builder::brush::Geometry) -> bool + 'a {
    move |brush_geometry: &&crate::geo_builder::brush::Geometry| {
        for plane in &brush_geometry.plane_geometry {
            if let Some(texture) = &plane.texture {
                if !texture_blacklist.is_blacklisted_brush(texture) {
                    return true;
                }
            } else {
                return true;
            }
        }

        false
    }
}
