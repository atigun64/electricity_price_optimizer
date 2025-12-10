use crate::optimizer_context::action::{
    constant::AssignedConstantAction, variable::AssignedVariableAction,
};

pub struct Schedule {
    pub constant_actions: Vec<AssignedConstantAction>,
    pub variable_actions: Vec<AssignedVariableAction>,
}
