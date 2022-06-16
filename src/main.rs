mod dp;
mod dp_traits;
mod collecting;
mod cache;
mod perf;

use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::rc::Rc;
use non_empty_vec::{ne_vec, NonEmpty};
use crate::dp_traits::DP;
use crate::dp::TopDownDP;
use crate::cache::{CacheAll, NoCache};
use crate::collecting::{Magma, ReduceByMagma, Reducer, Sum};
use crate::dp_traits::DPOwned;
use crate::perf::run_print_time;

struct PartialTopDownDP<'dp, I, R, M: Magma<R>, Solver: Fn(I) -> PartialProblemState<I, R>> {
    solver: Solver,
    compose_by: M,
    __phantoms: PhantomData<(&'dp (), I, R)>,
}

// Cache: CachePolicy<I, Rc<ProblemState<R, PartialProblemAnswerCombiner, I>>>,
impl<
    'dp,
    I: Copy + Debug,
    R: Copy + Debug,
    M: Copy + Magma<R>,
    Solver: Fn(I) -> PartialProblemState<I, R>,
> DP<'dp, I, R> for PartialTopDownDP<'dp, I, R, M, Solver> {
    fn dp(&'dp self, initial_index: I) -> R {
        use crate::perf::run_print_time;
        let solve_result_ref = (self.solver)(initial_index);
        // println!("computed {initial_index:?}, {:?}", &solve_result_ref);
        match solve_result_ref {
            PartialProblemState::Intermediate { dependent } => {
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
            PartialProblemState::Base { base_result } => {
                base_result.clone()
            }
        }

    }
}

fn simple_dp<'dp, I: Copy, R: Copy, M: Copy + Magma<R>, Solver: Fn(I) -> PartialProblemState<I, R>>(
    solver: Solver,
    compose_by: M,
) -> PartialTopDownDP<'dp, I, R, M, Solver> {
    PartialTopDownDP {
        solver,
        compose_by,
        __phantoms: Default::default()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PartialProblemState<I, R> {
    Base {
        base_result: R,
    },
    Intermediate {
        dependent: NonEmpty<I>,
    },
}

fn main() {
    // let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread", "vdso"]).build().unwrap();
    let f = |k: i32| {
        Rc::new(
            if k == 0 {
                ProblemState::Base {
                    base_result: 1
                }
            } else if k == 1 {
                ProblemState::Base {
                    base_result: 1
                }
            } else {
                ProblemState::Intermediate {
                    composer: |a: NonEmpty<i32>| a.iter().fold(0, |a, b| a + b),
                    dependent: ne_vec![k - 1, k - 2],
                }
            }
        )
    };

    run_dp(
        30,
        &TopDownDP::new(
            f,
            NoCache,
        )
    );
    run_dp(
        30,
        &TopDownDP::new(
            f,
            CacheAll::new(),
        )
    );

    {
        let dp = simple_dp(
            |k: i32| {
                if k == 0 {
                    PartialProblemState::Base {
                        base_result: 0
                    }
                } else if k == 1 {
                    PartialProblemState::Base {
                        base_result: 1
                    }
                } else {
                    PartialProblemState::Intermediate {
                        dependent: ne_vec![k - 1, k - 2]
                    }
                }
            },
            Sum::new(),
        );
        // FIXME: off-by-one error?
        println!("{}", run_print_time("the dp function", || dp.dp(31)));
    }
    /*
    match guard.report().build() {
        Ok(report) => {
            use pprof::protos::Message;
            let mut file = File::create("profile.pb").unwrap();
            let profile = report.pprof().unwrap();

            let mut content = Vec::new();
            profile.encode(&mut content).unwrap();
            file.write_all(&content).unwrap();

            // println!("report: {:?}", &report);
        }
        Err(_) => {}
    };

     */
}

fn run_dp<'a, Index, Output: 'a + Display>(index: Index, dp: &'a (impl DP<'a, Index, Output> + 'a)) {
    println!("{}", run_print_time("the dp function", || dp.dp(index)));
}

struct DPCopied<'r, 'a, Index, Answer: Copy, D>(D, PhantomData<(&'r (), &'a (), Index, Answer)>);

impl<'se, 'a, Index, Answer: 'se + Copy, D: 'se + DP<'se, Index, &'se Answer>> DPOwned<'se, Index, Answer> for DPCopied<'se, 'a, Index, Answer, D> {
    fn dp_owned(&'se self, index: Index) -> Answer {
        *self.0.dp(index)
    }
}

#[derive(Clone)]
pub enum ProblemState<A, F: Reducer<A>, I> {
    Intermediate {
        composer: F,
        dependent: NonEmpty<I>
    },
    Base {
        base_result: A,
    }
}

trait CollectingPolicy {
    type FoldType;
    fn join(lhs: Self::FoldType, rhs: Self::FoldType) -> Self::FoldType;

    fn empty_element() -> Self::FoldType;
}
