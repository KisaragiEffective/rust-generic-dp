use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::rc::Rc;
use non_empty_vec::NonEmpty;
use crate::cache::ArbitraryScopeCachePolicy;
use crate::collecting::Reducer;
use crate::dp::get_state::ProblemProxy;
use crate::dp::state::StateExtractor;
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
pub enum State<A, F, I> {
    Intermediate {
        composer: F,
        dependent: NonEmpty<I>
    },
    Base {
        base_result: A,
    }
}

impl<A: Clone, F, I> StateExtractor<A> for State<A, F, I> {
    fn get_value(&self) -> Option<A> {
        match self {
            State::Intermediate { .. } => None,
            State::Base { base_result } => Some(base_result.clone()),
        }
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
    Solver: ProblemProxy<I, Rc<State<R, PartialProblemAnswerCombiner, I>>, R>,
> DP<'dp, I, R> for TopDownDP<I, R, I, PartialProblemAnswerCombiner, Solver> {
    type State = Rc<State<R, PartialProblemAnswerCombiner, I>>;

    fn dp(&'dp self, initial_index: I) -> R {
        let solve_result = self.solver.compute(initial_index);
        let solve_result_ref = solve_result.as_ref();

        match solve_result_ref {
            State::Intermediate { composer, dependent } => {
                let len = dependent.len();
                let len = usize::from(len);
                let kk = crate::wrap_unsafe::maybe_garbage_vec::tap_garbage(len, |temp| {
                    for (i, x) in dependent.into_iter().enumerate() {
                        let lp = self.solver.get(*x).unwrap_or_else(|| {
                            let lm = self.dp(*x);
                            self.solver.update_cache(*x, lm.clone());
                            lm
                        });
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