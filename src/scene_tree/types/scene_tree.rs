use super::Actor;
use super::CollisionGeometry;
use super::VisualGeometry;

use crate::game_data::{EntityType, Properties, PropertyApplicationType};

#[derive(Debug)]
pub struct SceneTreeNode {
    pub origin: crate::Vector3,
    pub data: SceneTreeType,
}

#[derive(Debug)]
pub enum SceneTreeType {
    Actor(Actor, Vec<SceneTreeNode>),
    VisualGeometry(VisualGeometry),
    CollisionGeometry(CollisionGeometry),
}

impl SceneTreeNode {
    pub fn entity(
        name: String,
        origin: crate::Vector3,
        actor_type: EntityType,
        component_class: Option<String>,
        property_application_type: PropertyApplicationType,
        properties: Properties,
        children: Vec<SceneTreeNode>,
    ) -> SceneTreeNode {
        let data = SceneTreeType::Actor(
            Actor::new(name, actor_type, component_class, property_application_type, properties),
            children,
        );

        SceneTreeNode { origin, data }
    }

    pub fn visual_geometry(
        origin: crate::Vector3,
        visual_geometry: VisualGeometry,
    ) -> SceneTreeNode {
        let data = SceneTreeType::VisualGeometry(visual_geometry);
        SceneTreeNode { origin, data }
    }

    pub fn collision_geometry(
        origin: crate::Vector3,
        collision_geometry: CollisionGeometry,
    ) -> SceneTreeNode {
        let data = SceneTreeType::CollisionGeometry(collision_geometry);
        SceneTreeNode { origin, data }
    }
}
