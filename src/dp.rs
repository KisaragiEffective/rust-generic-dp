pub mod topdown;
pub mod simple;
pub mod traits;

use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fmt::Display;
use non_empty_vec::NonEmpty;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::rc::Rc;
use crate::cache::CachePolicy;
use crate::collecting::Magma;
use crate::{ProblemState, Reducer};
use crate::dp::simple::PartialTopDownDP;

fn simple_dp<'dp, I: Copy, R: Copy, M: Copy + Magma<R>, Solver: Fn(I) -> simple::State<I, R>>(
    solver: Solver,
    compose_by: M,
) -> PartialTopDownDP<'dp, I, R, M, Solver> {
    PartialTopDownDP {
        solver,
        compose_by,
        __phantoms: Default::default()
    }
}
