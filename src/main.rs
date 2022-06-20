#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery)]
mod dp;
mod collecting;
mod cache;
mod test;
mod wrap_unsafe;

use std::fmt::Display;
use non_empty_vec::{ne_vec, NonEmpty};
use crate::cache::CacheAll;
use crate::collecting::Sum;
use crate::dp::get_state::SolverFactory;
use crate::dp::simple::State;
use crate::dp::topdown;
use crate::dp::topdown::TopDownDP;
use crate::dp::traits::DP;
use crate::test::run_print_time;

#[allow(clippy::too_many_lines)]
fn main() {

}

fn run_dp<'a, Index, Output: 'a + Display>(index: Index, dp: &'a (impl DP<'a, Index, Output> + 'a)) {
    println!("{}", run_print_time("the dp function", || dp.dp(index)));
}
