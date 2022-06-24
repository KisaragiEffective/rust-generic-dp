












#[test]
#[allow(clippy::too_many_lines)]
fn run() {
    let f = |k: i32| {
        if k == 0 || k == 1 {
                complex::State::Base {
                    base_result: 1
                }
            }  else {
                complex::State::Intermediate {
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
        let dp = simple_dp(
            SolverFactory::function(
                |k: i32| {
                    if k == 0 || k == 1 {
                        simple::State::Base {
                            base_result: 1
                        }
                    } else {
                        simple::State::Intermediate {
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
        let dp = simple_dp(
            SolverFactory::function(
                |k: i32| {
                    if k == 0 || k == 1 {
                        simple::State::Base {
                            base_result: 1
                        }
                    } else {
                        simple::State::Intermediate {
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
        let dp = simple_dp(
            SolverFactory::function_with_cache(
                |k: i32| {
                    if k == 0 || k == 1 {
                        simple::State::Base {
                            base_result: 1
                        }
                    } else {
                        simple::State::Intermediate {
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
        let dp = simple_dp(
            SolverFactory::function_with_cache(
                |k: usize| {
                    if k == 0 || k == 1 {
                        simple::State::Base {
                            base_result: 1
                        }
                    } else {
                        simple::State::Intermediate {
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
        let dp = simple_dp(

            SolverFactory::function_with_cache(
                |k: usize| {
                    if k == 0 || k == 1 {
                        simple::State::Base {
                            base_result: 1
                        }
                    } else {
                        simple::State::Intermediate {
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
