use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::cache::CachePolicyByRef;

pub trait GetState<Input, State> {
    fn get(&self, input: Input) -> State;
    fn update_cache(&self, input: Input, get: State);
}

pub struct Factory;
impl Factory {
    pub fn function<Input, State>(f: impl Fn(Input) -> State) -> impl GetState<Input, State> {
        Function(f, PhantomData)
    }

    pub fn function_with_cache<Input: Clone, State: Clone>(f: impl Fn(Input) -> State, cache: impl CachePolicyByRef<Input, State>) -> impl GetState<Input, State> {
        FunctionWithCache {
            f,
            rc: RefCell::new(cache),
            _p: PhantomData
        }
    }

    pub fn function_with_cache_rc<Input, State>(f: impl Fn(Rc<Input>) -> Rc<State>, cache: impl CachePolicyByRef<Rc<Input>, Rc<State>>) -> impl GetState<Rc<Input>, Rc<State>> {
        FunctionWithCacheRc {
            f,
            cache_policy: RefCell::new(cache),
            _p: PhantomData
        }
    }
}

struct Function<F: Fn(Input) -> State, Input, State>(F, PhantomData<(Input, State)>);
impl<F: Fn(I) -> S, I, S> GetState<I, S> for Function<F, I, S> {
    fn get(&self, input: I) -> S {
        (self.0)(input)
    }

    #[inline]
    fn update_cache(&self, _input: I, _get: S) {
    }
}

struct FunctionWithCache<F: Fn(Input) -> State, CP: CachePolicyByRef<Input, State>, Input, State> {
    f: F,
    rc: RefCell<CP>,
    _p: PhantomData<(Input, State)>,
}

impl<F: Fn(I) -> S, CP: CachePolicyByRef<I, S>, I: Clone, S: Clone> GetState<I, S> for FunctionWithCache<F, CP, I, S> {
    fn get(&self, input: I) -> S {
        let cache_result = self.rc.borrow().get(&input).cloned();
        cache_result.unwrap_or_else(|| {
            let v = &(self.f)(input.clone());
            let v = v.clone();
            self.update_cache(input, v.clone());
            v
        })
    }

    fn update_cache(&self, input: I, get: S) {
        self.rc.borrow_mut().set(input, get);
    }
}

struct FunctionWithCacheRc<F: Fn(Rc<Input>) -> Rc<State>, CP: CachePolicyByRef<Rc<Input>, Rc<State>>, Input, State> {
    f: F,
    cache_policy: RefCell<CP>,
    _p: PhantomData<(Input, State)>,
}

impl<F: Fn(Rc<I>) -> Rc<S>, CP: CachePolicyByRef<Rc<I>, Rc<S>>, I, S> GetState<Rc<I>, Rc<S>> for FunctionWithCacheRc<F, CP, I, S> {
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