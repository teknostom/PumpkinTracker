use std::{collections::HashMap, fs, path::Path};

use serde::{Deserialize, Serialize};
use tree_sitter::{Parser, StreamingIterator};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClassMethods {
    class_name: String,
    methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum ImplementationStatus {
    Implemented,
    NotImplemented,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MethodTracking {
    method_name: String,
    status: ImplementationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClassTracking {
    class_name: String,
    methods: Vec<MethodTracking>,
    percentage_implemented: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AnalysisResult {
    classes: Vec<ClassTracking>,
    percentage_implemented: f32,
}

fn main() {
    let (rust_classes, java_classes) = extract_block_info();
    let tracking = analyze_implementation(&rust_classes, &java_classes);
    let analysis_result = AnalysisResult {
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
            0.0
        },
        classes: tracking.clone(),
    };
    let analysis_json = serde_json::to_string_pretty(&analysis_result).unwrap();
    fs::write("outputs/pumpkin_block_analysis.json", analysis_json).unwrap();
    println!("Wrote analysis to outputs/pumpkin_block_analysis.json");
}

fn extract_block_info() -> (Vec<ClassMethods>, Vec<ClassMethods>) {
    let rust_blocks = parse_rust_files("sources/Pumpkin/pumpkin/src/block/blocks");
    let rust_json = serde_json::to_string_pretty(&rust_blocks).unwrap();
    fs::write("outputs/pumpkin_blocks.json", rust_json).unwrap();
    println!("Extracted Pumpkin block info to outputs/pumpkin_blocks.json");
    let java_blocks = parse_java_files("sources/yarn/build/namedSrc/net/minecraft/block");
    let java_json = serde_json::to_string_pretty(&java_blocks).unwrap();
    fs::write("outputs/pumpkin_blocks_java.json", java_json).unwrap();
    println!("Extracted Pumpkin Java block info to outputs/pumpkin_blocks_java.json");
    (rust_blocks, java_blocks)
}

fn parse_rust_files(path: &str) -> Vec<ClassMethods> {
    let mut parser = Parser::new();

    let language = tree_sitter_rust::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Rust grammar");

    let query_str = r#"
       (impl_item
         type: (type_identifier) @type_name
         body: (declaration_list
           (function_item
             name: (identifier) @method_name)))
    "#;

    let query = tree_sitter::Query::new(&language.into(), query_str).unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut classes = HashMap::<String, Vec<String>>::new();

    let mut file_count = 0;

    walk_files(path, "rs", |_path, content| {
        file_count += 1;
        let tree = parser.parse(&content, None).unwrap();

        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(m) = matches.next() {
            let mut class_name = String::new();
            let mut method_name = String::new();

            for capture in m.captures {
                let text = capture.node.utf8_text(content.as_bytes()).unwrap();
                match capture.index {
                    0 => class_name = text.to_string(),
                    1 => method_name = text.to_string(),
                    _ => {}
                }
            }

            if !class_name.is_empty()
                && !method_name.is_empty()
                && is_core_block_method(&method_name)
            {
                classes
                    .entry(class_name)
                    .or_insert_with(Vec::new)
                    .push(method_name);
            }
        }
    });

    classes
        .into_iter()
        .map(|(class_name, methods)| ClassMethods {
            class_name,
            methods,
        })
        .collect()
}

fn parse_java_files(path: &str) -> Vec<ClassMethods> {
    let mut parser = Parser::new();

    let language = tree_sitter_java::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Java grammar");

    let query_str = r#"
       (class_declaration
         name: (identifier) @class_name
         body: (class_body
           (method_declaration
             name: (identifier) @method_name)))
    "#;

    let query = tree_sitter::Query::new(&language.into(), query_str).unwrap();
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut classes = HashMap::<String, Vec<String>>::new();

    walk_files(path, "java", |_path, content| {
        let tree = parser.parse(&content, None).unwrap();

        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(m) = matches.next() {
            let mut class_name = String::new();
            let mut method_name = String::new();

            for capture in m.captures {
                let text = capture.node.utf8_text(content.as_bytes()).unwrap();
                match capture.index {
                    0 => class_name = text.to_string(),
                    1 => method_name = text.to_string(),
                    _ => {}
                }
            }
            if !class_name.is_empty()
                && !method_name.is_empty()
                && is_core_block_method(&method_name)
            {
                classes
                    .entry(class_name)
                    .or_insert_with(Vec::new)
                    .push(method_name);
            }
        }
    });

    classes
        .into_iter()
        .map(|(class_name, methods)| ClassMethods {
            class_name,
            methods,
        })
        .collect()
}

fn analyze_implementation(
    rust_classes: &[ClassMethods],
    java_classes: &[ClassMethods],
) -> Vec<ClassTracking> {
    let mut tracking = Vec::new();
    let rust_map: HashMap<_, _> = rust_classes
        .iter()
        .map(|c| (c.class_name.as_str(), &c.methods))
        .collect();
    let java_map: HashMap<_, _> = java_classes
        .iter()
        .map(|c| (c.class_name.as_str(), &c.methods))
        .collect();

    for (class_name, java_methods) in &java_map {
        let empty_methods = Vec::new();
        let rust_methods = rust_map.get(class_name).cloned().unwrap_or(&empty_methods);
        let mut method_tracking = Vec::new();

        for java_method in java_methods.iter() {
            let status = if let Some(rust_equiv) = get_rust_equivalent(java_method.as_str()) {
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
                0.0
            },
        });
    }

    tracking
}

fn walk_files<F>(dir: &str, extension: &str, mut callback: F)
where
    F: FnMut(&Path, String),
{
    fn walk_recursive<F>(dir: &Path, extension: &str, callback: &mut F)
    where
        F: FnMut(&Path, String),
    {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk_recursive(&path, extension, callback);
                } else if path.extension().and_then(|s| s.to_str()) == Some(extension) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        callback(&path, content);
                    }
                }
            }
        }
    }

    walk_recursive(Path::new(dir), extension, &mut callback);
}

fn get_method_mapping() -> HashMap<&'static str, &'static str> {
    let mut mapping = HashMap::new();
    mapping.insert("can_place_at", "canPlaceAt");
    mapping.insert("on_place", "getPlacementState");
    mapping.insert("placed", "onPlaced");
    mapping.insert("broken", "onBreak");
    mapping.insert("normal_use", "onUse");
    mapping.insert("use_with_item", "onUseWithItem");
    mapping.insert("on_neighbor_update", "neighborUpdate");
    mapping.insert("on_scheduled_tick", "scheduledTick");
    mapping.insert("random_tick", "randomTick");
    mapping.insert("on_entity_collision", "onEntityCollision");
    mapping.insert("on_synced_block_event", "onSyncedBlockEvent");
    mapping.insert("on_state_replaced", "onStateReplaced");
    mapping.insert("get_state_for_neighbor_update", "getStateForNeighborUpdate");
    mapping.insert("emits_redstone_power", "emitsRedstonePower");
    mapping.insert("get_weak_redstone_power", "getWeakRedstonePower");
    mapping.insert("get_strong_redstone_power", "getStrongRedstonePower");
    mapping.insert("get_comparator_output", "getComparatorOutput");
    mapping.insert("prepare", "prepare"); // Direct match if exists
    mapping.insert("explode", "onExploded");

    // Additional Java methods that don't have direct Rust equivalents yet
    mapping.insert("on_block_added", "onBlockAdded");
    mapping.insert("has_random_ticks", "hasRandomTicks");
    mapping.insert("has_comparator_output", "hasComparatorOutput");
    mapping.insert("on_projectile_hit", "onProjectileHit");
    mapping.insert("create_block_entity", "createBlockEntity");

    mapping
}

fn is_core_block_method(method_name: &str) -> bool {
    let mapping = get_method_mapping();
    mapping.contains_key(method_name) || mapping.values().any(|&v| v == method_name)
}

fn get_rust_equivalent(java_method: &str) -> Option<&'static str> {
    let mapping = get_method_mapping();
    for (rust_method, java_equiv) in mapping {
        if java_equiv == java_method {
            return Some(rust_method);
        }
    }
    None
}
