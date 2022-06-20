use std::mem::MaybeUninit;
use non_empty_vec::NonEmpty;

// TODO: ミュータブル参照を引き回す関数を要求する代わりに、Fn() -> Rを要求し、それでmapした結果をwriteした上でassume_init()するべき
pub fn tap_non_empty_uninit_vec<R>(len: usize, tap: impl FnOnce(&mut NonEmpty<MaybeUninit<R>>)) -> NonEmpty<R> {
    assert!(len >= 1);
    let mut temp = Vec::with_capacity(len);
    temp.resize_with(len, || MaybeUninit::uninit());
    let mut temp = temp.try_into().unwrap();
    tap(&mut temp);
    // SAFETY: we verified that the temp vec is not empty, as we've enforced len must be non-zero.
    temp.into_iter().map(|a| unsafe { a.assume_init() }).collect::<Vec<_>>().try_into().unwrap()
}
