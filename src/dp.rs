use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt::Display;
use non_empty_vec::NonEmpty;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::rc::Rc;
use crate::cache::CachePolicy;
use crate::collecting::Magma;
use crate::ProblemState;
use crate::dp_traits::{DP, DPOwned};

pub struct TopDownDP<
    I,
    ProbAnswer,
    SRI,
    PartialProblemAnswerCombiner,
    Solver,
    Cache,
> {
    solver: Solver,
    // earn internal-mutability
    cache_policy: RefCell<Cache>,
    __phantoms: PhantomData<(I, ProbAnswer, SRI, PartialProblemAnswerCombiner)>
}

impl<
    I,
    ProbAnswer,
    SRI,
    PartialProblemAnswerCombiner,
    Solver,
    Cache,
> TopDownDP<I, ProbAnswer, SRI, PartialProblemAnswerCombiner, Solver, Cache> {
    pub(crate) fn new(solver: Solver, cache_policy: Cache) -> Self {
        Self {
            solver,
            cache_policy: RefCell::new(cache_policy),
            __phantoms: PhantomData,
        }
    }
}

/*
impl<
    'rr,
    I: 'rr,
    R: 'rr,
    PartialProblemAnswerCombiner: Fn(NonEmpty<Vec<&'rr R>>) -> &'rr R,
    Solver: Fn(&'rr I) -> ProblemState<&'rr R, PartialProblemAnswerCombiner, I>,
    BinaryCombiner: Fn(&'rr R, &'rr R) -> &'rr R
> DP<'rr, &'rr I, &'rr R> for TopDownDP<I, &'rr R, &'rr I, PartialProblemAnswerCombiner, Solver, BinaryCombiner> {
    fn dp(&'rr self, initial_index: &'rr I) -> &'rr R {
        let solve_result = (self.solver)(&initial_index);
        let mut result: Option<&R> = None;
        match solve_result {
            ProblemState::Intermediate { composer, dependent } => {
                let mut temp: Vec<&R> = vec![];
                for x in &dependent[0] {
                    let lp = self.dp(&x);
                    temp.push(lp);
                }
                result = Some(composer(NonEmpty::new(temp)));
            }
            ProblemState::Base { base_result } => {
                result = Some(base_result);
            }
        }

        result.unwrap()
    }
}
 */

impl<
    R,
    PartialProblemAnswerCombiner: Fn(NonEmpty<Vec<R>>) -> R,
    I
> AsRef<ProblemState<R, PartialProblemAnswerCombiner, I>> for ProblemState<R, PartialProblemAnswerCombiner, I> {
    fn as_ref(&self) -> &ProblemState<R, PartialProblemAnswerCombiner, I> {
        self
    }
}

impl<
    'dp,
    I: Copy,
    R: Clone,
    PartialProblemAnswerCombiner: Clone + Fn(NonEmpty<Vec<R>>) -> R,
    Solver: Fn(I) -> Rc<ProblemState<R, PartialProblemAnswerCombiner, I>>,
    Cache: CachePolicy<I, Rc<ProblemState<R, PartialProblemAnswerCombiner, I>>>,
> DP<'dp, I, R> for TopDownDP<I, R, I, PartialProblemAnswerCombiner, Solver, Cache> {
    fn dp(&'dp self, initial_index: I) -> R {
        use crate::perf::run_print_time;
        let xyy = {
            // あえてスコープを狭めないと関数スコープで生き続けてBorrowErrorでパニックする
            let ck = self.cache_policy.borrow();
            let xy = ck.get(&initial_index).cloned();
            xy
        };

        let solve_result = match xyy {
            None => {
                let value = (self.solver)(initial_index);
                self.cache_policy.borrow_mut().set(initial_index, Rc::new(value.as_ref().clone()));
                value
            },
            Some(a) => a.clone(),
        };

        let solve_result_ref = solve_result.as_ref();

        match solve_result_ref {
            ProblemState::Intermediate { composer, dependent } => {
                let inner = &dependent[0];
                let len = inner.len();
                let mut temp: Vec<MaybeUninit<R>> = Vec::with_capacity(len);
                temp.resize_with(len, || MaybeUninit::uninit());
                let mut i = 0;
                for x in inner {
                    let lp = self.dp(*x);
                    temp[i] = MaybeUninit::new(lp);
                    i += 1;
                }
                composer(NonEmpty::new(temp.into_iter().map(|a| unsafe { a.assume_init() }).collect()))
            }
            ProblemState::Base { base_result } => {
                base_result.clone()
            }
        }

    }
}