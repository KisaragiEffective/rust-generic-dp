use std::cell::RefCell;
use std::marker::PhantomData;
use crate::cache::ArbitraryScopeCachePolicy;
use crate::dp::state::StateExtractor;

pub trait ProblemProxy<Input, State, Answer> {
    /// always solve problem.
    fn compute(&self, input: Input) -> State;
    /// update cache if supported.
    /// if not supported, this is no-op.
    fn update_cache(&self, input: Input, answer: Answer);
    /// get answer without solving.
    /// if does not hit cache, this should be return None.
    /// if the implementor does not support cache, this may be always None.
    fn get(&self, input: Input) -> Option<Answer>;
}

// NOTE: this implementation is violate orphan rule.
// impl<Input, State, Answer, PP: ProblemProxy<Input, State, Answer>> Index<Input> for PP {
//     type Output = Option<Answer>;
//
//     fn index(&self, index: Input) -> &Self::Output {
//         self.get(index)
//     }
// }

pub struct SolverFactory;
impl SolverFactory {
    pub fn function<Input, State: StateExtractor<PartialAnswer>, PartialAnswer>(f: impl Fn(Input) -> State) -> impl ProblemProxy<Input, State, PartialAnswer> {
        Function(f, PhantomData)
    }

    pub fn function_with_cache<
        Input: Clone,
        State: StateExtractor<PartialAnswer>,
        PartialAnswer: Clone
    >(f: impl Fn(Input) -> State, cache: impl ArbitraryScopeCachePolicy<Input, PartialAnswer>) -> impl ProblemProxy<Input, State, PartialAnswer> {
        FunctionWithCache {
            f,
            cache_repo: RefCell::new(cache),
            _p: PhantomData
        }
    }
}

struct Function<F: Fn(Input) -> State, Input, State: StateExtractor<PA>, PA>(F, PhantomData<(Input, State, PA)>);
impl<F: Fn(I) -> S, I, S: StateExtractor<PA>, PA> ProblemProxy<I, S, PA> for Function<F, I, S, PA> {
    #[inline]
    fn compute(&self, input: I) -> S {
        (self.0)(input)
    }

    #[inline]
    fn update_cache(&self, _input: I, _get: PA) {
    }

    #[inline]
    fn get(&self, _input: I) -> Option<PA> {
        None
    }
}

struct FunctionWithCache<F: Fn(Input) -> State, CP: ArbitraryScopeCachePolicy<Input, PartialAnswer>, Input, State, PartialAnswer> {
    f: F,
    // earn inner mutability
    cache_repo: RefCell<CP>,
    _p: PhantomData<(Input, State, PartialAnswer)>,
}

impl<
    F: Fn(I) -> S,
    CP: ArbitraryScopeCachePolicy<I, PA>,
    I: Clone,
    S: StateExtractor<PA>,
    PA: Clone,
> ProblemProxy<I, S, PA> for FunctionWithCache<F, CP, I, S, PA> {
    fn compute(&self, input: I) -> S {
        let v = (self.f)(input.clone());
        v.get_value().map_or((), |pa| {
            self.update_cache(input, pa);
        });
        v
    }

    fn update_cache(&self, input: I, get: PA) {
        self.cache_repo.borrow_mut().set(input, get);
    }

    fn get(&self, input: I) -> Option<PA> {
        self.cache_repo.borrow().get(&input).cloned()
    }
}
