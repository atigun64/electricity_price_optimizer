use crate::action::constant::ConstantAction;
use crate::action::variable::{AssignedVariableAction};

pub struct Schedule {
    pub constant_actions: Vec<(i32, ConstantAction)>,
    pub variable_actions: Vec<AssignedVariableAction>,
}
