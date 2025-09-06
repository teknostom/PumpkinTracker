use crate::{ClassMethods, MinecraftComponent};
use std::collections::HashMap;
use tree_sitter::{Parser, StreamingIterator};

pub fn parse_rust_files<T: MinecraftComponent>(path: &str) -> Vec<ClassMethods> {
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

    crate::utils::walk_files(path, "rs", |_path, content| {
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

            if !class_name.is_empty() && !method_name.is_empty() && T::is_core_method(&method_name)
            {
                classes.entry(class_name).or_default().push(method_name);
            }
        }
    });

    classes
        .into_iter()
        .map(|(class_name, methods)| ClassMethods {
            class_name,
            methods,
            is_real_class: true, // Rust doesn't have interfaces in the same way as Java
        })
        .collect()
}
