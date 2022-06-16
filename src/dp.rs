use non_empty_vec::NonEmpty;
use std::marker::PhantomData;
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
> {
    solver: Solver,
    combiner: Combiner,
    __phantoms: PhantomData<(I, ProbAnswer, SRI, PartialProblemAnswerCombiner)>
}

impl<
    I,
    ProbAnswer,
    SRI,
    PartialProblemAnswerCombiner: Fn(NonEmpty<Vec<ProbAnswer>>) -> ProbAnswer,
    Solver: Fn(SRI) -> ProblemState<ProbAnswer, PartialProblemAnswerCombiner, I>,
    Combiner: Magma<ProbAnswer>
> TopDownDP<I, ProbAnswer, SRI, PartialProblemAnswerCombiner, Solver, Combiner> {
    pub(crate) fn new(solver: Solver, combiner: Combiner) -> Self {
        Self {
            solver, combiner, __phantoms: PhantomData
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
    'dp,
    I: Copy,
    R,
    PartialProblemAnswerCombiner: Fn(NonEmpty<Vec<R>>) -> R,
    Solver: Fn(I) -> ProblemState<R, PartialProblemAnswerCombiner, I>,
    BinaryCombiner: Magma<R>
> DP<'dp, I, R> for TopDownDP<I, R, I, PartialProblemAnswerCombiner, Solver, BinaryCombiner> {
    fn dp(&'dp self, initial_index: I) -> R {
        let solve_result = (self.solver)(initial_index);
        let mut result = None;
        match solve_result {
            ProblemState::Intermediate { composer, dependent } => {
                let mut temp = vec![];
                for x in &dependent[0] {
                    let lp = self.dp(*x);
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