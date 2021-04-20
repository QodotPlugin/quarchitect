use super::{CollisionType, ComponentType, EntityType, VisualType};

#[derive(Debug, Clone)]
pub struct WorldspawnLayer {
    pub name: String,
    pub texture: String,
    pub entity_type: EntityType,
    pub component_type: ComponentType,
    pub visual_type: VisualType,
    pub collision_type: CollisionType,
}

impl WorldspawnLayer {
    pub fn new(
        name: String,
        texture: String,
        entity_type: EntityType,
        component_type: ComponentType,
        visual_type: VisualType,
        collision_type: CollisionType,
    ) -> WorldspawnLayer {
        WorldspawnLayer {
            name,
            texture,
            entity_type,
            component_type,
            visual_type,
            collision_type,
        }
    }
}

impl Default for WorldspawnLayer {
    fn default() -> Self {
        let name = "Worldspawn Layer".into();
        let texture = "".into();
        let entity_type = EntityType::Class("".into());
        let component_type = ComponentType::None;
        let visual_type = VisualType::Mesh;
        let collision_type = CollisionType::Concave;

        WorldspawnLayer {
            name,
            texture,
            entity_type,
            component_type,
            visual_type,
            collision_type,
        }
    }
}
