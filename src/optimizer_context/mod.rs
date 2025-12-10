pub mod action;
pub mod battery;
pub mod prognoses;

use crate::optimizer_context::{
    action::{constant::ConstantAction, variable::VariableAction},
    battery::Battery,
    prognoses::Prognoses,
};

pub struct OptimizerContext {
    electricity_price: Prognoses,
    generated_electricity: Prognoses,
    beyond_control_consumption: Prognoses,
    batteries: Vec<Battery>,

    constant_actions: Vec<ConstantAction>,
    variable_actions: Vec<VariableAction>,
}
