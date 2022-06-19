pub trait DP<'r, Index, Answer> {
    type State;
    fn dp(&'r self, index: Index) -> Answer;
}
