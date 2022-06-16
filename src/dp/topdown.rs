use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::rc::Rc;
use non_empty_vec::NonEmpty;
use crate::cache::CachePolicy;
use crate::collecting::Reducer;
use crate::dp::traits::DP;

pub struct TopDownDP<
    I,
    ProbAnswer,
    SRI,
    PartialProblemAnswerCombiner,
    Solver,
    Cache,
> {
    solver: Solver,
    // earn internal-mutability
    cache_policy: RefCell<Cache>,
    __phantoms: PhantomData<(I, ProbAnswer, SRI, PartialProblemAnswerCombiner)>
}

#[derive(Clone)]
pub enum State<A, F: Reducer<A>, I> {
    Intermediate {
        composer: F,
        dependent: NonEmpty<I>
    },
    Base {
        base_result: A,
    }
}

impl<
    I,
    ProbAnswer,
    SRI,
    PartialProblemAnswerCombiner,
    Solver,
    Cache,
> TopDownDP<I, ProbAnswer, SRI, PartialProblemAnswerCombiner, Solver, Cache> {
    pub(crate) fn new(solver: Solver, cache_policy: Cache) -> Self {
        Self {
            solver,
            cache_policy: RefCell::new(cache_policy),
            __phantoms: PhantomData,
        }
    }
}

impl<
    'dp,
    I: Copy,
    R: Clone,
    PartialProblemAnswerCombiner: Clone + Reducer<R>,
    Solver: Fn(I) -> Rc<State<R, PartialProblemAnswerCombiner, I>>,
    Cache: CachePolicy<I, Rc<State<R, PartialProblemAnswerCombiner, I>>>,
> DP<'dp, I, R> for TopDownDP<I, R, I, PartialProblemAnswerCombiner, Solver, Cache> {
    type State = Rc<State<R, PartialProblemAnswerCombiner, I>>;

    fn dp(&'dp self, initial_index: I) -> R {
        use crate::perf::run_print_time;
        let xyy = {
            // あえてスコープを狭めないと関数スコープで生き続けてBorrowErrorでパニックする
            let ck = self.cache_policy.borrow();
            let xy = ck.get(&initial_index).cloned();
            xy
        };

        let solve_result = match xyy {
            None => {
                let value = (self.solver)(initial_index);
                self.cache_policy.borrow_mut().set(initial_index, Rc::new(value.as_ref().clone()));
                value
            },
            Some(a) => a.clone(),
        };

        let solve_result_ref = solve_result.as_ref();

        match solve_result_ref {
            State::Intermediate { composer, dependent } => {
                let len = dependent.len();
                let len = usize::from(len);
                let mut temp: Vec<MaybeUninit<R>> = Vec::with_capacity(len);
                temp.resize_with(len, || MaybeUninit::uninit());
                let mut i = 0;
                for x in dependent {
                    let lp = self.dp(*x);
                    temp[i] = MaybeUninit::new(lp);
                    i += 1;
                }
                composer.reduce((temp.into_iter().map(|a| unsafe { a.assume_init() }).collect::<Vec<_>>()).try_into().unwrap())
            }
            State::Base { base_result } => {
                base_result.clone()
            }
        }

    }
}