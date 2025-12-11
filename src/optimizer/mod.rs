use crate::optimizer::MCMF::MinCostFlow;
use crate::variable_maker::VariableMaker;
use crate::simulated_annealing::state::State;


struct optimizer {
    pub mf:MinCostFlow,
    pub variable_map:VariableMaker,
}

impl optimizer {
    pub fn new(state: &State) -> Self {
        let variable_maker = VariableMaker::new(state.get_context());
        let mf = MinCostFlow::new(variable_maker.get_variable_count(), variable_maker.SOURCE, variable_maker.SINK);

        
    }
}