use std::marker::PhantomData;
use crate::cache::CachePolicy;

pub trait GetState<Input, State> {
    fn get(&self, input: Input) -> State;
}

pub struct Factory;
impl Factory {
    pub fn function<Input, State>(f: impl Fn(Input) -> State) -> impl GetState<Input, State> {
        Function(f, PhantomData)
    }

    pub fn function_with_cache<Input, State>(f: impl Fn(Input) -> State, cache: impl CachePolicy<Input, State>) -> impl GetState<Input, State> {
        FunctionWithCache(f, cache, PhantomData)
    }
}

struct Function<F: Fn(Input) -> State, Input, State>(F, PhantomData<(Input, State)>);
impl<F: Fn(I) -> S, I, S> GetState<I, S> for Function<F, I, S> {
    fn get(&self, input: I) -> S {
        (self.0)(input)
    }
}

struct FunctionWithCache<F: Fn(Input) -> State, CP: CachePolicy<Input, State>, Input, State>(F, CP, PhantomData<(Input, State)>);
impl<F: Fn(I) -> S, CP: CachePolicy<I, S>, I, S> GetState<I, S> for FunctionWithCache<F, CP, I, S> {
    fn get(&self, input: I) -> S {
        (self.0)(input)
    }
}
