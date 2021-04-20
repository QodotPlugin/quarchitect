use crate::game_data::{EntityType, Properties, PropertyApplicationType};

#[derive(Debug)]
pub struct Actor {
    pub name: String,
    pub entity_type: EntityType,
    pub component_class: Option<String>,
    pub property_application_type: PropertyApplicationType,
    pub properties: Properties,
}

impl Actor {
    pub fn new(
        name: String,
        entity_type: EntityType,
        component_class: Option<String>,
        property_application_type: PropertyApplicationType,
        properties: Properties,
    ) -> Actor {
        Actor {
            name,
            entity_type,
            component_class,
            property_application_type,
            properties,
        }
    }
}
