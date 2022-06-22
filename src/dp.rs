pub mod complex;
pub mod simple;
pub mod traits;
pub mod get_state;
pub mod state;

use crate::collecting::Magma;
use crate::dp::get_state::ProblemProxy;
use crate::dp::simple::State;
use crate::dp::simple::SimpleDPRunner;

// TODO: キャッシュの取得を外側から差し込めるようなインターフェースにする
pub(crate) fn simple_dp<'dp, I: Copy, R: Copy, M: Copy + Magma<R>, Solver: ProblemProxy<I, State<I, R>, PartialAnswer>, PartialAnswer>(
    solver: Solver,
    compose_by: M,
) -> SimpleDPRunner<'dp, I, R, M, Solver> {
    SimpleDPRunner {
        solver,
        compose_by,
        __phantoms: Default::default()
    }
}
