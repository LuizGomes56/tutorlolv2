use super::*;

impl Generator for InfinityEdge {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
