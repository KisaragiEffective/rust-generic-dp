use std::marker::PhantomData;
use std::ops::Add;
use non_empty_vec::NonEmpty;

pub trait Magma<A> {
    fn combine(&self, lhs: A, rhs: A) -> A;
}

/// An marker trait implies that the operation is commutative.
/// In other words, the implementation must be follow below property:
///     f(a, b) == f(b, a)
///
/// where f is `Magma::combine` implementation, `a: A`, and `b: A`.
/// It is useful when combining in parallel.
pub trait Commutative {}

impl<F: Copy + FnOnce(A, A) -> A, A: Copy> Magma<A> for F {
    fn combine(&self, lhs: A, rhs: A) -> A {
        self(lhs, rhs)
    }
}

pub trait Monoid<A>: Magma<A> {
    fn empty_element() -> A;
}

#[derive(Copy, Clone)]
pub struct Sum<T>(PhantomData<T>);

impl<T> Sum<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Add<Output=T>> Magma<T> for Sum<T> {
    fn combine(&self, lhs: T, rhs: T) -> T {
        lhs + rhs
    }
}

pub trait Reducer<T> {
    fn reduce(&self, reduce_source: NonEmpty<T>) -> T;
}

pub struct ReduceByMagma<T, M>(M, PhantomData<T>);

impl<T, M: Clone> Clone for ReduceByMagma<T, M> {
    fn clone(&self) -> Self {
        Self::new(self.0.clone())
    }
}

impl<T, M: Copy> Copy for ReduceByMagma<T, M> {}

impl<T, M> ReduceByMagma<T, M> {
    pub fn new(magma: M) -> Self {
        Self(magma, PhantomData)
    }
}

impl<T: Copy, M: Magma<T>> Reducer<T> for ReduceByMagma<T, M> {
    fn reduce(&self, reduce_source: NonEmpty<T>) -> T {
        // dont fold with the first element.
        // SAFETY: we know the iterator is also non-empty
        reduce_source.clone().into_iter().reduce(|a, b| self.0.combine(a, b)).unwrap()
    }
}

impl<T: Clone, F: Copy + FnOnce(NonEmpty<T>) -> T> Reducer<T> for F {
    fn reduce(&self, reduce_source: NonEmpty<T>) -> T {
        (&self(reduce_source)).clone().clone()
    }
}

/*
struct Max<T: Ord>;
struct MaxOption<T: PartialOrd>;
struct Min<T: Ord>;
struct MinOption<T: PartialOrd>;
*/

/*
struct Product<T: std::ops::Mul>;
struct BitAnd<T: std::ops::BitAnd>;
struct BirOr<T: std::ops::BitOr>;
struct BitXor<T: std::ops::BitXor>;
*/
