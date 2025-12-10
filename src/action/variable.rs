pub struct VariableAction {
    pub start: i32,
    pub end: i32,
    pub total_consumption: i32,
    pub max_consumption: i32,
    id: i32,
}
pub struct AssignedVariableAction {
    action: VariableAction,
    consumption: Vec<i32>,
}
