mod dp;
mod dp_traits;
mod collecting;
mod cache;
mod perf;

use std::fmt::Display;
use std::marker::PhantomData;
use std::rc::Rc;
use non_empty_vec::NonEmpty;
use crate::dp_traits::DP;
use crate::dp::TopDownDP;
use crate::cache::{CacheAll, NoCache};
use crate::collecting::Magma;
use crate::dp_traits::DPOwned;
use crate::perf::run_print_time;

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
                    composer: |a| a[0].iter().fold(0, |a, b| a + b),
                    dependent: NonEmpty::new(vec![k - 1, k - 2])
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
pub enum ProblemState<A, F: Fn(NonEmpty<Vec<A>>) -> A, I> {
    Intermediate {
        composer: F,
        dependent: NonEmpty<Vec<I>>
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
