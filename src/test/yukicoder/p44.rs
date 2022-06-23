use non_empty_vec::ne_vec;
use crate::cache::CacheArray;
use crate::collecting::Sum;
use crate::dp::get_state::SolverFactory;
use crate::dp::simple::State::{Base, Intermediate};
use crate::dp::simple_dp;
use crate::dp::traits::DP;

///
///
#[test]
fn test() {
    assert_eq!(inner_test(2), 2);
    assert_eq!(inner_test(3), 3);
    assert_eq!(inner_test(5), 8);
}

fn inner_test(index: usize) -> u64 {
    simple_dp(
        SolverFactory::function_with_cache(|i| {
            if (i == 1) {
                Base {
                    base_result: 1
                }
            } else if (i == 2) {
                Base {
                    base_result: 2
                }
            } else {
                Intermediate {
                    dependent: ne_vec![i - 1, i - 2]
                }
            }
        },             CacheArray::<_, 51>::new()),
        Sum::new()
    ).dp(index)
}
