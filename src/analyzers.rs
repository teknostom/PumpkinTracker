use crate::{
    AnalysisResult, ClassMethods, ClassTracking, ImplementationStatus, MethodTracking,
    MinecraftComponent,
};
use std::{collections::HashMap, fs};

pub fn analyze_implementation<T: MinecraftComponent>(
    rust_classes: &[ClassMethods],
    java_classes: &[ClassMethods],
) -> Vec<ClassTracking> {
    let mut tracking = Vec::new();
    let rust_map: HashMap<_, _> = rust_classes
        .iter()
        .map(|c| (c.class_name.to_lowercase(), &c.methods))
        .collect();

    let real_java_classes: Vec<_> = java_classes
        .iter()
        .filter(|c| c.is_real_class)
        .filter(|c| c.methods.len() > 0)
        .filter(|c| c.class_name != "AbstractBlockState")
        .filter(|c| !c.class_name.ends_with("BlockEntity"))
        .collect();

    let java_map: HashMap<_, _> = real_java_classes
        .iter()
        .map(|c| (c.class_name.as_str(), &c.methods))
        .collect();

    for (class_name, java_methods) in &java_map {
        let empty_methods = Vec::new();
        let rust_methods = rust_map
            .get(&class_name.to_lowercase())
            .cloned()
            .unwrap_or(&empty_methods);
        let mut method_tracking = Vec::new();

        for java_method in java_methods.iter() {
            let status = if let Some(rust_equiv) = T::get_rust_equivalent(java_method.as_str()) {
                if rust_methods.contains(&rust_equiv.to_string()) {
                    ImplementationStatus::Implemented
                } else {
                    ImplementationStatus::NotImplemented
                }
            } else {
                ImplementationStatus::NotImplemented
            };
            method_tracking.push(MethodTracking {
                method_name: java_method.to_string(),
                status,
            });
        }

        let implemented_count = method_tracking
            .iter()
            .filter(|m| m.status == ImplementationStatus::Implemented)
            .count();
        let total_count = method_tracking.len();

        tracking.push(ClassTracking {
            class_name: class_name.to_string(),
            methods: method_tracking,
            percentage_implemented: if total_count > 0 {
                (implemented_count as f32 / total_count as f32) * 100.0
            } else {
                100.0
            },
        });
    }

    tracking
}

pub fn run_analysis<T: MinecraftComponent>() -> AnalysisResult {
    let (rust_classes, java_classes) = crate::extractors::extract_component_info::<T>();
    let mut tracking = analyze_implementation::<T>(&rust_classes, &java_classes);

    tracking.sort_by(|a, b| a.class_name.cmp(&b.class_name));

    let analysis_result = AnalysisResult {
        component_type: T::component_type().to_string(),
        percentage_implemented: if !tracking.is_empty() {
            let total_methods: usize = tracking.iter().map(|c| c.methods.len()).sum();
            let implemented_methods: usize = tracking
                .iter()
                .map(|c| {
                    c.methods
                        .iter()
                        .filter(|m| m.status == ImplementationStatus::Implemented)
                        .count()
                })
                .sum();
            (implemented_methods as f32 / total_methods as f32) * 100.0
        } else {
            100.0
        },
        classes: tracking,
    };

    let analysis_json = serde_json::to_string_pretty(&analysis_result).unwrap();
    fs::write(&T::output_files().analysis, analysis_json).unwrap();
    println!(
        "Wrote {} analysis to {}",
        T::component_type(),
        T::output_files().analysis
    );

    analysis_result
}
