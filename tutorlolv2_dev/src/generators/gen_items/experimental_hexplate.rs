use super::*;

impl Generator for ExperimentalHexplate {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
