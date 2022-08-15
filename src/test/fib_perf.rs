use non_empty_vec::{ne_vec, NonEmpty};
use crate::cache::{CacheAll, CacheArray, CacheVec};
use crate::collecting::Sum;
use crate::dp::complex::ComplexDP;
use crate::dp::get_state::SolverFactory;
use crate::dp::simple_dp;
use crate::dp::traits::DP;
use crate::test::{run_dp, run_print_time};

#[test]
#[allow(clippy::too_many_lines)]
fn run() {
    let f = |k: i32| {
        use crate::dp::complex::State;
        if k == 0 || k == 1 {
                State::Base {
                    base_result: 1
                }
            }  else {
                State::Intermediate {
                    composer: |a: NonEmpty<i32>| a.iter().sum(),
                    dependent: ne_vec![k - 1, k - 2],
                }
            }
    };

    run_dp(
        30,
        &ComplexDP::new(
            SolverFactory::function(f)
        )
    );
    run_dp(
        30,
        &ComplexDP::new(
            SolverFactory::function_with_cache(f, CacheAll::new())
        )
    );

    {
        use crate::dp::simple::State;
        let dp = simple_dp(
            SolverFactory::function(
                |k: i32| {
                    if k == 0 || k == 1 {
                        State::Base {
                            base_result: 1
                        }
                    } else {
                        State::Intermediate {
                            dependent: ne_vec![k - 1, k - 2]
                        }
                    }
                }
            ),
            Sum::new(),
        );
        println!("{}", run_print_time("simple dp w/o memoize, Sum struct", || dp.dp(30)));
    }

    {
        use crate::dp::simple::State;
        let dp = simple_dp(
            SolverFactory::function(
                |k: i32| {
                    if k == 0 || k == 1 {
                        State::Base {
                            base_result: 1
                        }
                    } else {
                        State::Intermediate {
                            dependent: ne_vec![k - 1, k - 2]
                        }
                    }
                }
            ),
            |a, b| a + b,
        );
        println!("{}", run_print_time("simple dp w/o memoize, function reducer", || dp.dp(30)));
    }

    {
        use crate::dp::simple::State;
        let dp = simple_dp(
            SolverFactory::function_with_cache(
                |k: i32| {
                    if k == 0 || k == 1 {
                        State::Base {
                            base_result: 1
                        }
                    } else {
                        State::Intermediate {
                            dependent: ne_vec![k - 1, k - 2]
                        }
                    }
                },
                CacheAll::new()
            ),
            Sum::new(),
        );
        println!("{}", run_print_time("simple dp w/ cache by hashmap", || dp.dp(30)));
    }

    {
        use crate::dp::simple::State;
        let dp = simple_dp(
            SolverFactory::function_with_cache(
                |k: usize| {
                    if k == 0 || k == 1 {
                        State::Base {
                            base_result: 1
                        }
                    } else {
                        State::Intermediate {
                            dependent: ne_vec![k - 1, k - 2]
                        }
                    }
                },
                CacheVec::new()
            ),
            Sum::new(),
        );
        println!("{}", run_print_time("simple dp w/ cache by vec", || dp.dp(30)));
    }


    {
        use crate::dp::simple::State;
        let dp = simple_dp(
            SolverFactory::function_with_cache(
                |k: usize| {
                    if k == 0 || k == 1 {
                        State::Base {
                            base_result: 1
                        }
                    } else {
                        State::Intermediate {
                            dependent: ne_vec![k - 1, k - 2]
                        }
                    }
                },
                CacheArray::<_, 31>::new()
            ),
            Sum::new(),
        );
        println!("{}", run_print_time("simple dp w/ cache by array", || dp.dp(30)));
    }

    {
        let res = run_print_time("honest dp w/ cache by array", || {
            let mut dp_cache = [0u64; 31];
            dp_cache[0] = 1;
            dp_cache[1] = 1;
            for i in 2..31 {
                dp_cache[i] = dp_cache[i - 1] + dp_cache[i - 2];
            }

            dp_cache[30]
        });

        println!("{res}");
    }
}
