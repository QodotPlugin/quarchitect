use crate::Vector2;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TextureInfo(pub HashMap<String, Texture>);

#[derive(Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Texture {
        Texture { width, height }
    }

    pub fn size(&self) -> Vector2 {
        Vector2::new(self.width as f32, self.height as f32)
    }
}

#[derive(Debug)]
pub struct TextureBlacklist {
    pub brush: Vec<String>,
    pub plane: Vec<String>,
}

impl Default for TextureBlacklist {
    fn default() -> TextureBlacklist {
        let brush = Vec::new();
        let plane = Vec::new();
        TextureBlacklist { brush, plane }
    }
}

impl TextureBlacklist {
    pub fn new(brush: Vec<String>, plane: Vec<String>) -> TextureBlacklist {
        TextureBlacklist { brush, plane }
    }

    pub fn is_blacklisted_brush(&self, texture: &str) -> bool {
        for brush_texture in self.brush.iter() {
            if texture == brush_texture {
                return true;
            }
        }

        false
    }

    pub fn is_blacklisted_plane(&self, texture: &str) -> bool {
        for plane_texture in self.plane.iter() {
            if texture == plane_texture {
                return true;
            }
        }

        false
    }
}
