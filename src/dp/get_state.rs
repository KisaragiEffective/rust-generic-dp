use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
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

pub struct SolverFactory;
impl SolverFactory {
    pub fn function<Input, State: StateExtractor<PartialAnswer>, PartialAnswer>(f: impl Fn(Input) -> State) -> impl ProblemProxy<Input, State, PartialAnswer> {
        Function(f, PhantomData)
    }

    pub fn function_with_cache<
        Input: Clone,
        State: Clone + StateExtractor<PartialAnswer>,
        PartialAnswer: Clone
    >(f: impl Fn(Input) -> State, cache: impl ArbitraryScopeCachePolicy<Input, PartialAnswer>) -> impl ProblemProxy<Input, State, PartialAnswer> {
        FunctionWithCache {
            f,
            cache_repo: RefCell::new(cache),
            _p: PhantomData
        }
    }

    pub fn function_with_cache_rc<Input, State>(f: impl Fn(Rc<Input>) -> Rc<State>, cache: impl ArbitraryScopeCachePolicy<Rc<Input>, Rc<State>>) -> impl GetState<Rc<Input>, Rc<State>> {
        FunctionWithCacheRc {
            f,
            cache_policy: RefCell::new(cache),
            _p: PhantomData
        }
    }
}

struct Function<F: Fn(Input) -> State, Input, State: StateExtractor<PA>, PA>(F, PhantomData<(Input, State, PA)>);
impl<F: Fn(I) -> S, I, S: StateExtractor<PA>, PA> ProblemProxy<I, S, PA> for Function<F, I, S, PA> {
    fn compute(&self, input: I) -> S {
        (self.0)(input)
    }

    #[inline]
    fn update_cache(&self, _input: I, _get: PA) {
    }

    fn get(&self, _input: I) -> Option<PA> {
        None
    }
}

struct FunctionWithCache<F: Fn(Input) -> State, CP: ArbitraryScopeCachePolicy<Input, PartialAnswer>, Input, State, PartialAnswer> {
    f: F,
    cache_repo: RefCell<CP>,
    _p: PhantomData<(Input, State, PartialAnswer)>,
}

impl<
    F: Fn(I) -> S,
    CP: ArbitraryScopeCachePolicy<I, PA>,
    I: Clone,
    S: Clone + StateExtractor<PA>,
    PA: Clone,
> ProblemProxy<I, S, PA> for FunctionWithCache<F, CP, I, S, PA> {
    fn compute(&self, input: I) -> S {
        let v = &(self.f)(input.clone());
        let v = v.clone();
        v.get_value().map_or_else(|| (), |pa| {
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

struct FunctionWithCacheRc<F: Fn(Rc<Input>) -> Rc<State>, CP: ArbitraryScopeCachePolicy<Rc<Input>, Rc<State>>, Input, State> {
    f: F,
    cache_policy: RefCell<CP>,
    _p: PhantomData<(Input, State)>,
}

impl<F: Fn(Rc<I>) -> Rc<S>, CP: ArbitraryScopeCachePolicy<Rc<I>, Rc<S>>, I, S> GetState<Rc<I>, Rc<S>> for FunctionWithCacheRc<F, CP, I, S> {
    fn get(&self, input: Rc<I>) -> Rc<S> {
        let x = self.cache_policy.borrow().get(&input).cloned();
        x.unwrap_or_else(|| {
            let v = (self.f)(input.clone());
            self.update_cache(input, v.clone());
            v
        })
    }

    fn update_cache(&self, input: Rc<I>, get: Rc<S>) {
        self.cache_policy.borrow_mut().set(input, get);
    }
}