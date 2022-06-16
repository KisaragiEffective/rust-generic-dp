use crate::DPCopied;

pub trait DP<'r, Index, Answer> {
    type State;
    fn dp(&'r self, index: Index) -> Answer;
}

trait DPMut<Index, Answer> {
    fn dp_mut(&mut self, index: Index) -> Answer;
}

pub trait DPOwned<'se, Index, Answer> {
    fn dp_owned(&'se self, index: Index) -> Answer;
}
