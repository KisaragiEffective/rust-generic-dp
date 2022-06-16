mod dp;
mod dp_traits;
mod collecting;
mod cache;

use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, Index};
use std::time::Instant;
use non_empty_vec::NonEmpty;
use dp_traits::DP;
use dp::TopDownDP;
use crate::cache::{CacheAll, NoCache};
use crate::dp_traits::DPOwned;

fn main() {
    let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread", "vdso"]).build().unwrap();
    run_dp(
        30,
        &TopDownDP::new(
            |k: i32| {
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
            },
            crate::collecting::Sum::new(),
            NoCache,
        )
    );
    run_dp(
        30,
        &TopDownDP::new(
            |k: i32| {
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
            },
            crate::collecting::Sum::new(),
            CacheAll::new(),
        )
    );
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
}

fn check<'a, F: 'a + FnOnce() -> T, T>(f: F) -> T {
    let start = Instant::now();
    let t = f();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    t
}
fn run_dp<'a, Index, Output: 'a + Display>(index: Index, dp: &'a (impl DP<'a, Index, Output> + 'a)) {
    let p = dp.dp(index);
    println!("{}", check::<'a, _, _>(|| p));
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

/*
struct Max<T: Ord>;
struct MaxOption<T: PartialOrd>;
struct Min<T: Ord>;
struct MinOption<T: PartialOrd>;
*/

#[derive(Default)]
struct Sum<T: std::ops::Add>(PhantomData<T>);
/*
struct Product<T: std::ops::Mul>;
struct BitAnd<T: std::ops::BitAnd>;
struct BirOr<T: std::ops::BitOr>;
struct BitXor<T: std::ops::BitXor>;
*/
