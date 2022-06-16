use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use non_empty_vec::NonEmpty;
use crate::dp::traits::DP;
use crate::collecting::Magma;
use crate::collecting::ReduceByMagma;

pub(crate) struct PartialTopDownDP<'dp, I, R, M: Magma<R>, Solver: Fn(I) -> State<I, R>> {
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
    Solver: Fn(I) -> State<I, R>,
> DP<'dp, I, R> for PartialTopDownDP<'dp, I, R, M, Solver> {
    fn dp(&'dp self, initial_index: I) -> R {
        use crate::perf::run_print_time;
        let solve_result_ref = (self.solver)(initial_index);
        // println!("computed {initial_index:?}, {:?}", &solve_result_ref);
        match solve_result_ref {
            State::Intermediate { dependent } => {
                // let dependent = dbg!(dependent);
                let smaller_indexes = &dependent;
                let len = smaller_indexes.len().into();
                let mut buffer: Vec<MaybeUninit<R>> = Vec::with_capacity(len);
                buffer.resize_with(len, || MaybeUninit::uninit());
                let mut i = 0;
                for smaller_index in smaller_indexes {
                    let lp = self.dp(*smaller_index);
                    buffer[i] = MaybeUninit::new(lp);
                    i += 1;
                }
                let collected_values = buffer.into_iter().map(|a| unsafe { a.assume_init() }).collect::<Vec<_>>();
                let (_, collected_values) = (initial_index, collected_values);
                let reducer = ReduceByMagma::new(self.compose_by);
                use crate::collecting::Reducer;
                let reduce_result = reducer.reduce(collected_values.try_into().unwrap());
                reduce_result
            }
            State::Base { base_result } => {
                base_result.clone()
            }
        }

    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum State<I, R> {
    Base {
        base_result: R,
    },
    Intermediate {
        dependent: NonEmpty<I>,
    },
}
