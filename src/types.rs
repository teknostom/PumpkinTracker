use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMethods {
    pub class_name: String,
    pub methods: Vec<String>,
    pub is_real_class: bool,
}

#[derive(Debug)]
pub struct ClassInfo {
    pub name: String,
    pub methods: Vec<String>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImplementationStatus {
    Implemented,
    NotImplemented,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodTracking {
    pub method_name: String,
    pub status: ImplementationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassTracking {
    pub class_name: String,
    pub methods: Vec<MethodTracking>,
    pub percentage_implemented: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub component_type: String,
    pub classes: Vec<ClassTracking>,
    pub percentage_implemented: f32,
}

#[derive(Debug)]
pub struct ComponentOutputFiles {
    pub rust_methods: String,
    pub java_methods: String,
    pub analysis: String,
}
