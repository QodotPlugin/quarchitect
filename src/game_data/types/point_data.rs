use super::{ComponentType, EntityType, Properties};

#[derive(Debug, Copy, Clone)]
pub enum PropertyApplicationType {
    Properties,
    Dictionary,
    Metadata,
}

#[derive(Debug, Clone)]
pub struct PointData {
    pub entity_type: EntityType,
    pub component_type: ComponentType,
    pub property_application_type: PropertyApplicationType,
    pub properties: Properties,
}

impl PointData {
    pub fn new(
        entity_type: EntityType,
        component_type: ComponentType,
        property_application_type: PropertyApplicationType,
        properties: Properties,
    ) -> PointData {
        PointData {
            entity_type,
            component_type,
            property_application_type,
            properties,
        }
    }
}

impl Default for PointData {
    fn default() -> Self {
        let entity_type = EntityType::Placeholder;
        let component_type = ComponentType::None;
        let property_application_type = PropertyApplicationType::Properties;
        let properties = Properties::default();

        PointData {
            entity_type,
            component_type,
            property_application_type,
            properties,
        }
    }
}
