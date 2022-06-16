use std::marker::PhantomData;
use std::ops::Add;

pub trait Magma<A> {
    fn combine(&self, lhs: A, rhs: A) -> A;
}

impl<F: Copy + FnOnce(A, A) -> A, A: Copy> Magma<A> for F {
    fn combine(&self, lhs: A, rhs: A) -> A {
        self(lhs, rhs)
    }
}

pub trait Monoid<A>: Magma<A> {
    fn empty_element() -> A;
}

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
