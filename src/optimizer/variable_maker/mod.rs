use std::collections::HashMap;
use crate::optimizer_context::OptimizerContext;
use crate::variable_maker::DoubleSidedHash::BiMap;

const MINUTES_PER_DAY: u32 = 60 * 24;

pub const SOURCE: u32 = 0;
pub const SINK: u32 = 1;
pub const NETWORK: u32 = 2;
pub const GENERATOR: u32 = 3;

pub type ItemId = u32;
pub type Timestamp = u32;

pub type PersistentVariableIndex = (ItemId, Timestamp);
pub type NonpersistentVariableIndex = ItemId;

pub struct VariableMaker { 
    context: OptimizerContext,
    
    persistent_variable_indices: BiMap<PersistentVariableIndex, usize>,
    nonpersistent_variable_indices: BiMap<NonpersistentVariableIndex, usize>,
}
impl VariableMaker {
    pub fn new(context: &OptimizerContext) -> Self {
        let mut persistent = BiMap::new();
        let mut nonpersistent = BiMap::new();

        let mut idx: usize = 4;

        for var_action in context.get_variable_actions().iter().enumerate() {
            let item_id = var_action.get_id() as u32;
            for t in (0, MINUTES_PER_DAY) {
                persistent.insert((item_id, t), idx);
                idx += 1;
            }
        }
        for const_action in context.get_constant_actions().iter().enumerate() {
            let item_id = const_action.get_id() as u32;
            for t in (0, MINUTES_PER_DAY) {
                persistent.insert((item_id, t), idx);
                idx += 1;
            }
        }
        for battery in context.get_batteries().iter().enumerate() {
            let item_id = battery.get_id() as u32;
            for t in (0, MINUTES_PER_DAY) {
                persistent.insert((item_id, t), idx);
                idx += 1;
            }
        }

        Self {
            context,
            persistent_variable_indices: persistent,
            nonpersistent_variable_indices: nonpersistent,
        }
    }

    pub fn get_action_variable_index(&self, item_id: ItemId, timestamp: Timestamp) -> Option<usize> {
        self.persistent_variable_indices.get_by_left(&(item_id, timestamp)).cloned()
    }
    pub fn get_battery_variable_index(&self, item_id: ItemId, timestamp: Timestamp) -> Option<usize> {
        self.persistent_variable_indices.get_by_left(&(item_id, timestamp)).cloned()
    }
    pub fn get_constant_action_variable_index(&self, item_id: ItemId, timestamp: Timestamp) -> Option<usize> {
        self.persistent_variable_indices.get_by_left(&(item_id, timestamp)).cloned()
    }
}
