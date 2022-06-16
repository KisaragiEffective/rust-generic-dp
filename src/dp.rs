pub mod topdown;
pub mod simple;
pub mod traits;

use crate::collecting::Magma;
use crate::dp::simple::State;
use crate::dp::simple::PartialTopDownDP;

pub(crate) fn simple_dp<'dp, I: Copy, R: Copy, M: Copy + Magma<R>, Solver: Fn(I) -> State<I, R>>(
    solver: Solver,
    compose_by: M,
) -> PartialTopDownDP<'dp, I, R, M, Solver> {
    PartialTopDownDP {
        solver,
        compose_by,
        __phantoms: Default::default()
    }
}
