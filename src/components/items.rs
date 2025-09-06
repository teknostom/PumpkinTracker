use crate::MinecraftComponent;
use std::collections::HashMap;

pub struct ItemComponent;

impl MinecraftComponent for ItemComponent {
    fn component_type() -> &'static str {
        "items"
    }

    fn rust_source_path() -> &'static str {
        "sources/Pumpkin/pumpkin/src/item/items"
    }

    fn java_source_path() -> &'static str {
        "sources/yarn/build/namedSrc/net/minecraft/item"
    }

    fn method_mapping() -> HashMap<&'static str, &'static str> {
        HashMap::new()
    }
}
