use crate::{
    action::{constant::ConstantAction, variable::VariableAction},
    prognoses::Prognoses,
};

pub struct EnvironmentData {
    electricity_price: Prognoses,
    generated_electricity: Prognoses,
    beyond_control_consumption: Prognoses,

    constant_actions: Vec<ConstantAction>,
    variable_actions: Vec<VariableAction>,
}
