use crate::optimizer_context::action::{
    constant::AssignedConstantAction, variable::VariableAction,
};

pub struct State {
    constant_actions: Vec<AssignedConstantAction>,
    variable_actions: Vec<VariableAction>,
}

impl State {
    pub fn get_constant_actions(&self) -> &Vec<AssignedConstantAction> {
        &self.constant_actions
    }
    pub fn get_constant_actions_mut(&mut self) -> &mut Vec<AssignedConstantAction> {
        &mut self.constant_actions
    }

    pub fn set_constant_actions(&mut self, constant_actions: Vec<AssignedConstantAction>) {
        self.constant_actions = constant_actions;
    }
}
