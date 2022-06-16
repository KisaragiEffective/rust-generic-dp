use std::fmt::Display;
use std::time::Instant;

pub fn run_print_time<'a, F: 'a + FnOnce() -> T, T>(message: &'a impl Display, f: F) -> T {
    let start = Instant::now();
    let t = f();
    let duration = start.elapsed();

    println!("Time elapsed in {str} is: {:?}", duration);
    t
}
