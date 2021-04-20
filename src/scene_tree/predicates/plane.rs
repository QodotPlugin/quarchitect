pub fn has_texture<'a>(
    texture: &'a Option<String>,
) -> impl Fn(&&crate::geo_builder::brush_plane::Geometry) -> bool + 'a {
    move |plane_geometry: &&crate::geo_builder::brush_plane::Geometry| {
        &plane_geometry.texture == texture
    }
}
