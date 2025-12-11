use crate::optimizer::MCMF::MinCostFlow;
use crate::variable_maker::VariableMaker;


struct optimizer {
    pub mf:MinCostFlow,
    pub variable_map:VariableMaker,
}

impl optimizer {
    pub fn new() -> Self {
        Self {
            mf: MinCostFlow::new(),
            variable_maker: VariableMaker::new(),
        }
    }
}