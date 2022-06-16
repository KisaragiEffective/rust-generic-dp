use std::time::Instant;

pub fn run_print_time<'a, F: 'a + FnOnce() -> T, T>(str: &'a str, f: F) -> T {
    let start = Instant::now();
    let t = f();
    let duration = start.elapsed();

    println!("Time elapsed in {str} is: {:?}", duration);
    t
}
