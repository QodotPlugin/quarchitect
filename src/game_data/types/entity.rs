use super::{
    BrushData, CollisionType, ComponentType, EntityType, PointData, Properties,
    PropertyApplicationType, VisualType,
};

#[derive(Debug)]
pub enum EntityData {
    Point(PointData),
    Brush(PointData, BrushData),
}

#[derive(Debug)]
pub struct Entity {
    pub classname: String,
    pub data: EntityData,
}

impl Entity {
    pub fn point(
        classname: String,
        entity_type: EntityType,
        component_type: ComponentType,
        property_application_type: PropertyApplicationType,
        properties: Properties,
    ) -> Entity {
        let data = EntityData::Point(PointData::new(entity_type, component_type, property_application_type, properties));
        Entity { classname, data }
    }

    pub fn brush(
        classname: String,
        entity_type: EntityType,
        component_type: ComponentType,
        property_application_type: PropertyApplicationType,
        properties: Properties,
        visual_type: VisualType,
        collision_type: CollisionType,
    ) -> Entity {
        let data = EntityData::Brush(
            PointData::new(
                entity_type,
                component_type,
                property_application_type,
                properties,
            ),
            BrushData::new(visual_type, collision_type),
        );

        Entity { classname, data }
    }
}
