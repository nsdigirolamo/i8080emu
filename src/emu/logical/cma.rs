use crate::{
    emu::State,
    parsers::{logical::cma::CMA, register::Register},
};

pub fn execute_cma(state: &mut State, cma: CMA) {
    match cma {
        CMA::ComplementAccumulator => {
            let data = state.get_register(&Register::A);

            let result = !data;

            // No condition flags are affected.
            state.set_register(&Register::A, result);
        }
    }
}
