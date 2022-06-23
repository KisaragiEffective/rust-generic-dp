use std::ops::Deref;
#[allow(clippy::module_name_repetitions)]
pub trait StateExtractor<T> {
    /// returns computed result.
    /// if computed, the this method returns Some(result).
    /// If not, returns None.
    fn get_value(&self) -> Option<T>;
}

impl<P: Deref<Target=State>, State: StateExtractor<Answer>, Answer> StateExtractor<Answer> for P {
    fn get_value(&self) -> Option<Answer> {
        self.deref().deref().get_value()
    }
}