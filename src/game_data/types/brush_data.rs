use super::{CollisionType, VisualType};

#[derive(Debug)]
pub struct BrushData {
    pub visual_type: VisualType,
    pub collision_type: CollisionType,
}

impl BrushData {
    pub fn new(visual_type: VisualType, collision_type: CollisionType) -> BrushData {
        BrushData {
            visual_type,
            collision_type,
        }
    }
}

impl Default for BrushData {
    fn default() -> Self {
        let visual_type = VisualType::None;
        let collision_type = CollisionType::None;

        BrushData {
            visual_type,
            collision_type,
        }
    }
}
