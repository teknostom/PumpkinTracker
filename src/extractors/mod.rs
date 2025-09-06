pub mod java_extractor;
pub mod rust_extractor;

pub use java_extractor::*;
pub use rust_extractor::*;

use crate::{ClassMethods, MinecraftComponent};
use std::fs;

pub fn extract_component_info<T: MinecraftComponent>() -> (Vec<ClassMethods>, Vec<ClassMethods>) {
    let rust_classes = parse_rust_files::<T>(T::rust_source_path());
    let rust_json = serde_json::to_string_pretty(&rust_classes).unwrap();
    fs::write(&T::output_files().rust_methods, rust_json).unwrap();
    println!(
        "Extracted Pumpkin {} info to {}",
        T::component_type(),
        T::output_files().rust_methods
    );

    let java_classes = parse_java_files::<T>(T::java_source_path());
    let java_json = serde_json::to_string_pretty(&java_classes).unwrap();
    fs::write(&T::output_files().java_methods, java_json).unwrap();
    println!(
        "Extracted Java {} info to {}",
        T::component_type(),
        T::output_files().java_methods
    );

    (rust_classes, java_classes)
}
