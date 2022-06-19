#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]
mod dp;
mod collecting;
mod cache;
mod perf;
mod wrap_unsafe;

use std::borrow::Borrow;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::rc::Rc;
use non_empty_vec::{ne_vec, NonEmpty};
use crate::cache::{CacheAll, NoCache};
use crate::collecting::{Magma, ReduceByMagma, Reducer, Sum};
use crate::dp::get_state::SolverFactory;
use crate::dp::simple::State;
use crate::dp::topdown;
use crate::dp::topdown::TopDownDP;
use crate::dp::traits::{DP, DPOwned};
use crate::perf::run_print_time;

#[allow(clippy::too_many_lines)]
fn main() {
    let f = |k: i32| {
        (
            if k == 0 || k == 1 {
                topdown::State::Base {
                    base_result: 1
                }
            }  else {
                topdown::State::Intermediate {
                    composer: |a: NonEmpty<i32>| a.iter().sum(),
                    dependent: ne_vec![k - 1, k - 2],
                }
            }
        )
    };

    run_dp(
        30,
        &TopDownDP::new(
            SolverFactory::function(f)
        )
    );
    run_dp(
        30,
        &TopDownDP::new(
            SolverFactory::function_with_cache(f, CacheAll::new())
        )
    );

    {
        let dp = dp::simple_dp(
            SolverFactory::function(
                |k: i32| {
                    if k == 0 || k == 1 {
                        State::Base {
                            base_result: 1
                        }
                    } else {
                        State::Intermediate {
                            dependent: ne_vec![k - 1, k - 2]
                        }
                    }
                }
            ),
            Sum::new(),
        );
        println!("{}", run_print_time("simple dp w/o memoize", || dp.dp(30)));
    }

    {
        let dp = dp::simple_dp(
            SolverFactory::function_with_cache(
                |k: i32| {
                    if k == 0 || k == 1 {
                        State::Base {
                            base_result: 1
                        }
                    } else {
                        State::Intermediate {
                            dependent: ne_vec![k - 1, k - 2]
                        }
                    }
                },
                CacheAll::new()
            ),
            Sum::new(),
        );
        println!("{}", run_print_time("simple dp w/ cache by hashmap", || dp.dp(30)));
    }

    {
        let dp = dp::simple_dp(
            SolverFactory::function_with_cache(
                |k: usize| {
                    if k == 0 || k == 1 {
                        State::Base {
                            base_result: 1
                        }
                    } else {
                        State::Intermediate {
                            dependent: ne_vec![k - 1, k - 2]
                        }
                    }
                },
                cache::CacheVec::new()
            ),
            Sum::new(),
        );
        println!("{}", run_print_time("simple dp w/ cache by vec", || dp.dp(30)));
    }


    {
        let dp = dp::simple_dp(

            SolverFactory::function_with_cache(
                |k: usize| {
                    if k == 0 || k == 1 {
                        State::Base {
                            base_result: 1
                        }
                    } else {
                        State::Intermediate {
                            dependent: ne_vec![k - 1, k - 2]
                        }
                    }
                },
                cache::CacheArray::<_, 31>::new()
            ),
            Sum::new(),
        );
        println!("{}", run_print_time("simple dp w/ cache by array", || dp.dp(30)));
    }
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

trait CollectingPolicy {
    type FoldType;
    fn join(lhs: Self::FoldType, rhs: Self::FoldType) -> Self::FoldType;

    fn empty_element() -> Self::FoldType;
}
