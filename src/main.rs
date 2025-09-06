use pumpkin_tracker::analyzers::run_analysis;
use pumpkin_tracker::components::{BlockComponent, EntityComponent, ItemComponent};

fn main() {
    // Analyze blocks
    println!("=== Analyzing Blocks ===");
    let block_analysis = run_analysis::<BlockComponent>();
    println!(
        "Block implementation: {:.1}%\n",
        block_analysis.percentage_implemented
    );

    // Analyze items
    println!("=== Analyzing Items ===");
    let item_analysis = run_analysis::<ItemComponent>();
    println!(
        "Item implementation: {:.1}%\n",
        item_analysis.percentage_implemented
    );

    // Analyze entities
    println!("=== Analyzing Entities ===");
    let entity_analysis = run_analysis::<EntityComponent>();
    println!(
        "Entity implementation: {:.1}%\n",
        entity_analysis.percentage_implemented
    );

    // Overall summary
    let overall_percentage = /* ( */
        block_analysis.percentage_implemented;
    // + item_analysis.percentage_implemented
    // + entity_analysis.percentage_implemented
    // ) / 3.0;

    println!("=== Overall Summary ===");
    println!("Blocks: {:.1}%", block_analysis.percentage_implemented);
    println!("Items: {:.1}%", item_analysis.percentage_implemented);
    println!("Entities: {:.1}%", entity_analysis.percentage_implemented);
    println!("Average: {:.1}%", overall_percentage);
}
