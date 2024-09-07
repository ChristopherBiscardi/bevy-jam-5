use bevy::prelude::*;

use crate::persistent_id::PersistentId;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Inventory>()
            .register_type::<Item>();
    }
}

#[derive(Debug, Reflect, PartialEq)]
pub enum ProcessedState {
    Unprocessed,
    Processed,
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Inventory {
    pub max_item_count: usize,
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn has_available_space(&self) -> bool {
        self.items.len() < self.max_item_count
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Item {
    pub name: String,
    pub owner: Option<PersistentId>,
    pub state: ProcessedState,
}

impl Item {
    pub fn process(&mut self) -> &mut Self {
        self.state = ProcessedState::Processed;
        self
    }
}
