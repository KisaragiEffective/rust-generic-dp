use std::mem::MaybeUninit;

pub fn tap_garbage<R>(len: usize, mut tap: impl FnMut(&mut Vec<MaybeUninit<R>>)) -> Vec<R> {
    let mut temp: Vec<MaybeUninit<R>> = Vec::with_capacity(len);
    temp.resize_with(len, || MaybeUninit::uninit());
    tap(&mut temp);
    temp.into_iter().map(|a| unsafe { a.assume_init() }).collect()
}