use non_empty_vec::NonEmpty;
use std::marker::PhantomData;
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
    Combiner,
    Cache,
> {
    solver: Solver,
    combiner: Combiner,
    cache_policy: Cache,
    __phantoms: PhantomData<(I, ProbAnswer, SRI, PartialProblemAnswerCombiner)>
}

impl<
    I,
    ProbAnswer,
    SRI,
    PartialProblemAnswerCombiner,
    Solver,
    Combiner,
    Cache,
> TopDownDP<I, ProbAnswer, SRI, PartialProblemAnswerCombiner, Solver, Combiner, Cache> {
    pub(crate) fn new(solver: Solver, combiner: Combiner, cache_policy: Cache) -> Self {
        Self {
            solver,
            combiner,
            cache_policy,
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
    Solver: Fn(I) -> PS,
    PS: Clone + AsRef<ProblemState<R, PartialProblemAnswerCombiner, I>>,
    BinaryCombiner: Magma<R>,
    Cache: CachePolicy<I, PS>,
> DP<'dp, I, R> for TopDownDP<I, R, I, PartialProblemAnswerCombiner, Solver, BinaryCombiner, Cache> {
    fn dp(&'dp self, initial_index: I) -> R {
        let solve_result = match self.cache_policy.get(&initial_index) {
            None => (self.solver)(initial_index).clone(),
            Some(a) => a.clone(),
        };

        match solve_result {
            ProblemState::Intermediate { composer, dependent } => {
                let inner = &dependent[0];
                let len = inner.len();
                // TODO: this can be uninited, later we overwrite anyway.
                let mut temp = Vec::with_capacity(len);
                for x in inner {
                    let lp = self.dp(*x);
                    temp.push(lp);
                }
                composer(NonEmpty::new(temp))
            }
            ProblemState::Base { base_result } => {
                base_result.clone()
            }
        }

    }
}