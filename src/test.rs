mod fib_perf;

use std::fmt::Display;
use std::time::Instant;
use crate::dp::traits::DP;

pub fn run_print_time<'a, F: 'a + FnOnce() -> T, T>(message: &'a (impl Display + ?Sized), f: F) -> T {
    let start = Instant::now();
    let t = f();
    let duration = start.elapsed();

    println!("Time elapsed in {message} is: {:?}", duration);
    t
}

fn run_dp<'a, Index, Output: 'a + Display>(index: Index, dp: &'a (impl DP<'a, Index, Output> + 'a)) {
    println!("{}", run_print_time("the dp function", || dp.dp(index)));
}
