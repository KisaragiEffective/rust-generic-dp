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


