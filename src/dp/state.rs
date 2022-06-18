use std::ops::Deref;

pub trait StateExtractor<T> {
    fn get_value(&self) -> Option<T>;
}

impl<P: Deref<Target=State>, State: StateExtractor<Answer>, Answer> StateExtractor<Answer> for P {
    fn get_value(&self) -> Option<Answer> {
        self.deref().deref().get_value()
    }
}