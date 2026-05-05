use super::*;

impl Generator for LuckyDice {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
