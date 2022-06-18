use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::rc::Rc;
use non_empty_vec::NonEmpty;
use crate::cache::CachePolicyByRef;
use crate::collecting::Reducer;
use crate::dp::get_state::GetState;
use crate::dp::traits::DP;

pub struct TopDownDP<
    I,
    ProbAnswer,
    SRI,
    PartialProblemAnswerCombiner,
    Solver
> {
    solver: Solver,
    __phantoms: PhantomData<(I, ProbAnswer, SRI, PartialProblemAnswerCombiner)>
}

#[derive(Clone)]
pub enum State<A, F: Reducer<A>, I> {
    Intermediate {
        composer: F,
        dependent: NonEmpty<I>
    },
    Base {
        base_result: A,
    }
}

impl<
    I,
    ProbAnswer,
    SRI,
    PartialProblemAnswerCombiner,
    Solver
> TopDownDP<I, ProbAnswer, SRI, PartialProblemAnswerCombiner, Solver> {
    pub(crate) fn new(solver: Solver) -> Self {
        Self {
            solver,
            __phantoms: PhantomData,
        }
    }
}

impl<
    'dp,
    I: Copy,
    R: Clone,
    PartialProblemAnswerCombiner: Clone + Reducer<R>,
    Solver: GetState<I, Rc<State<R, PartialProblemAnswerCombiner, I>>>,
> DP<'dp, I, R> for TopDownDP<I, R, I, PartialProblemAnswerCombiner, Solver> {
    type State = Rc<State<R, PartialProblemAnswerCombiner, I>>;

    fn dp(&'dp self, initial_index: I) -> R {
        let solve_result = self.solver.get(initial_index);
        let solve_result_ref = solve_result.as_ref();

        match solve_result_ref {
            State::Intermediate { composer, dependent } => {
                let len = dependent.len();
                let len = usize::from(len);
                let kk = crate::wrap_unsafe::maybe_garbage_vec::tap_garbage(len, |temp| {
                    for (i, x) in dependent.into_iter().enumerate() {
                        let lp = self.dp(*x);
                        temp[i] = MaybeUninit::new(lp);
                    }
                });
                composer.reduce(kk.try_into().unwrap())
            }
            State::Base { base_result } => {
                base_result.clone()
            }
        }

    }
}