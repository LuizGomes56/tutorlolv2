use super::*;

impl Generator for JackOfAllTrades {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Jack */
            .min(1) /* Passive */
            .end()
    }
}
