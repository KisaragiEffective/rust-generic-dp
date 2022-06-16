use std::marker::PhantomData;
use std::ops::Add;

trait Magma<A> {
    fn combine(&self, lhs: A, rhs: A) -> A;
}

impl<F: FnOnce(A, A) -> A, A> Magma<A> for F {
    fn combine(&self, lhs: A, rhs: A) -> A {
        self(lhs, rhs)
    }
}

trait Monoid<A>: Magma<A> {
    fn empty_element() -> A;
}

struct Sum<T>(PhantomData<T>);
impl<T> Sum<T> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Add> Magma<T> for Sum<T> {
    fn combine(&self, lhs: T, rhs: T) -> T {
        lhs + rhs
    }
}


