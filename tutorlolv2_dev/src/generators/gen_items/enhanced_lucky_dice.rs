use super::*;

impl Generator for EnhancedLuckyDice {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
