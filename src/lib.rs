use std::collections::HashMap;

pub mod analyzers;
pub mod components;
pub mod extractors;
pub mod types;
pub mod utils;

pub use analyzers::*;
pub use extractors::*;
pub use types::*;
pub use utils::*;

// Core trait for analyzing Minecraft components
pub trait MinecraftComponent {
    /// Get the component type (e.g., "blocks", "items", "entities")
    fn component_type() -> &'static str;

    /// Get the Rust source path for this component
    fn rust_source_path() -> &'static str;

    /// Get the Java source path for this component
    fn java_source_path() -> &'static str;

    /// Get the mapping between Rust and Java method names
    fn method_mapping() -> HashMap<&'static str, &'static str>;

    /// Check if a method is considered a core method for this component
    fn is_core_method(method_name: &str) -> bool {
        let mapping = Self::method_mapping();
        mapping.contains_key(method_name) || mapping.values().any(|&v| v == method_name)
    }

    /// Get the Rust equivalent of a Java method name
    fn get_rust_equivalent(java_method: &str) -> Option<&'static str> {
        let mapping = Self::method_mapping();
        for (rust_method, java_equiv) in mapping {
            if java_equiv == java_method {
                return Some(rust_method);
            }
        }
        None
    }

    /// Get output file names for this component
    fn output_files() -> ComponentOutputFiles {
        ComponentOutputFiles {
            rust_methods: format!("outputs/pumpkin_{}.json", Self::component_type()),
            java_methods: format!("outputs/pumpkin_{}_java.json", Self::component_type()),
            analysis: format!("outputs/pumpkin_{}_analysis.json", Self::component_type()),
        }
    }
}
