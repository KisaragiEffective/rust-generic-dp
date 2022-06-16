pub mod topdown;
pub mod simple;
pub mod traits;
pub mod get_state;

use crate::collecting::Magma;
use crate::dp::get_state::GetState;
use crate::dp::simple::State;
use crate::dp::simple::PartialTopDownDP;

// TODO: キャッシュの取得を外側から差し込めるようなインターフェースにする
pub(crate) fn simple_dp<'dp, I: Copy, R: Copy, M: Copy + Magma<R>, Solver: GetState<I, State<I, R>>>(
    solver: Solver,
    compose_by: M,
) -> PartialTopDownDP<'dp, I, R, M, Solver> {
    PartialTopDownDP {
        solver,
        compose_by,
        __phantoms: Default::default()
    }
}
