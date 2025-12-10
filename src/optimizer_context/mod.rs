pub mod action;
pub mod battery;
pub mod prognoses;

use std::rc::Rc;

use crate::optimizer_context::{
    action::{constant::ConstantAction, variable::VariableAction},
    battery::Battery,
    prognoses::{ElectricityPrognoses, PricePrognoses},
};

pub struct OptimizerContext {
    electricity_price: PricePrognoses,
    generated_electricity: ElectricityPrognoses,
    beyond_control_consumption: ElectricityPrognoses,

    batteries: Vec<Battery>,

    constant_actions: Vec<Rc<ConstantAction>>,
    variable_actions: Vec<Rc<VariableAction>>,
}
impl OptimizerContext {
    pub fn get_constant_actions(&self) -> &Vec<Rc<ConstantAction>> {
        &self.constant_actions
    }
}
