mod color;
mod entity;
mod texture;
mod vertex;

pub type Vector2 = glam::Vec2;
pub type Vector3 = glam::Vec3;
pub type Mat2 = glam::Mat2;
pub type Quat = glam::Quat;

pub use color::Color;
pub use texture::Texture;
pub use texture::TextureBlacklist;
pub use texture::TextureInfo;
pub use vertex::Vertex;
