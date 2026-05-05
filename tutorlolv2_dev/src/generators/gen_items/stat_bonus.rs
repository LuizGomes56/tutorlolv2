use super::*;

impl Generator for StatBonus {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
