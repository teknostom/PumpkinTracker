use crate::MinecraftComponent;
use std::collections::HashMap;

pub struct BlockComponent;

impl MinecraftComponent for BlockComponent {
    fn component_type() -> &'static str {
        "blocks"
    }

    fn rust_source_path() -> &'static str {
        "sources/Pumpkin/pumpkin/src/block/blocks"
    }

    fn java_source_path() -> &'static str {
        "sources/yarn/build/namedSrc/net/minecraft/block"
    }

    fn method_mapping() -> HashMap<&'static str, &'static str> {
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
        mapping.insert("prepare", "prepare");
        mapping.insert("explode", "onExploded");
        mapping.insert("on_block_added", "onBlockAdded");
        mapping.insert("has_random_ticks", "hasRandomTicks");
        mapping.insert("has_comparator_output", "hasComparatorOutput");
        mapping.insert("on_projectile_hit", "onProjectileHit");
        mapping.insert("create_block_entity", "createBlockEntity");
        mapping
    }
}
