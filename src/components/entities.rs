use crate::MinecraftComponent;
use std::collections::HashMap;

pub struct EntityComponent;

impl MinecraftComponent for EntityComponent {
    fn component_type() -> &'static str {
        "entities"
    }

    fn rust_source_path() -> &'static str {
        "sources/Pumpkin/pumpkin/src/entity/entities"
    }

    fn java_source_path() -> &'static str {
        "sources/yarn/build/namedSrc/net/minecraft/entity"
    }

    fn method_mapping() -> HashMap<&'static str, &'static str> {
        HashMap::new()
    }
}
