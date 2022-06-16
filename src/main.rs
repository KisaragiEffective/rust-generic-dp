mod dp;
mod dp_traits;
mod collecting;

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Deref, Index};
use non_empty_vec::NonEmpty;
use dp_traits::DP;
use dp::TopDownDP;
use crate::dp_traits::DPOwned;

fn main() {
    println!("Hello, world!");
    // let x = Fib.solve(5, (), &Box::new(|a, b| a + b));
    let owo = TopDownDP::new(
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
        |a: i32, b: i32| a + b,
    );

    let x = owo.dp(5);
    println!("{x}")
}

struct DPCopied<'r, 'a, Index, Answer: Copy, D>(D, PhantomData<(&'r (), &'a (), Index, Answer)>);

impl<'se, 'a, Index, Answer: 'se + Copy, D: 'se + DP<'se, Index, &'se Answer>> DPOwned<'se, Index, Answer> for DPCopied<'se, 'a, Index, Answer, D> {
    fn dp_owned(&'se self, index: Index) -> Answer {
        *self.0.dp(index)
    }
}

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
