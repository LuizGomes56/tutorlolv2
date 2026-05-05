use super::*;

impl Generator for TwilightsEdge {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
