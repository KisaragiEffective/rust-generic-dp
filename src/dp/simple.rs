use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use non_empty_vec::NonEmpty;
use crate::dp::traits::DP;
use crate::collecting::Magma;
use crate::collecting::ReduceByMagma;
use crate::dp::get_state::ProblemProxy;
use crate::dp::state::StateExtractor;

/// an simpler DP solver.
/// the answer is always reduced by `compose_by`.
/// It is preferred if you can use this strategy,
/// as it may be simple and faster.
pub struct SimpleDPRunner<'dp, I, R, M: Magma<R>, Solver> {
    pub(super) solver: Solver,
    pub(super) compose_by: M,
    pub(super) __phantoms: PhantomData<(&'dp (), I, R)>,
}

// Cache: CachePolicy<I, Rc<ProblemState<R, PartialProblemAnswerCombiner, I>>>,
impl<
    'dp,
    I: Copy + Debug,
    R: Copy + Debug,
    M: Copy + Magma<R>,
    Solver: ProblemProxy<I, State<I, R>, R>
> DP<'dp, I, R> for SimpleDPRunner<'dp, I, R, M, Solver> {
    type State = State<I, R>;

    fn dp(&'dp self, initial_index: I) -> R {
        let solve_result_ref = self.solver.compute(initial_index);
        // println!("computed {initial_index:?}, {:?}", &solve_result_ref);
        match solve_result_ref {
            State::Intermediate { dependent } => {
                // let dependent = dbg!(dependent);
                let len = dependent.len().into();
                let collected_values = crate::wrap_unsafe::maybe_garbage_vec::tap_non_empty_uninit_vec(len, |temp| {
                    for (i, x) in dependent.iter().enumerate() {
                        let lp = self.solver.get(*x).unwrap_or_else(|| {
                            let lm = self.dp(*x);
                            self.solver.update_cache(*x, lm);
                            lm
                        });
                        temp[i] = MaybeUninit::new(lp);
                    }
                });
                let reducer = ReduceByMagma::new(self.compose_by);
                use crate::collecting::Reducer;
                reducer.reduce(collected_values)
            }
            State::Base { base_result } => {
                base_result
            }
        }

    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum State<I, R> {
    Base {
        base_result: R,
    },
    Intermediate {
        dependent: NonEmpty<I>,
    },
}

impl<I, R: Clone> StateExtractor<R> for State<I, R> {
    fn get_value(&self) -> Option<R> {
        match self {
            State::Base { base_result } => Some(base_result.clone()),
            State::Intermediate { .. } => None
        }
    }
}