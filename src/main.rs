#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]
mod dp;
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
use crate::cache::{CacheAll, NoCache};
use crate::collecting::{Magma, ReduceByMagma, Reducer, Sum};
use crate::dp::simple::State;
use crate::dp::topdown;
use crate::dp::topdown::TopDownDP;
use crate::dp::traits::{DP, DPOwned};
use crate::perf::run_print_time;

fn main() {
    // let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread", "vdso"]).build().unwrap();
    let f = |k: i32| {
        Rc::new(
            if k == 0 {
                topdown::State::Base {
                    base_result: 1
                }
            } else if k == 1 {
                topdown::State::Base {
                    base_result: 1
                }
            } else {
                topdown::State::Intermediate {
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
        let dp = dp::simple_dp(
            |k: i32| {
                if k == 0 {
                    State::Base {
                        base_result: 0
                    }
                } else if k == 1 {
                    State::Base {
                        base_result: 1
                    }
                } else {
                    State::Intermediate {
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

trait CollectingPolicy {
    type FoldType;
    fn join(lhs: Self::FoldType, rhs: Self::FoldType) -> Self::FoldType;

    fn empty_element() -> Self::FoldType;
}
