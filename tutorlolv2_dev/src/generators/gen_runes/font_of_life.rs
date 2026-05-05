use super::*;

impl Generator for FontOfLife {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Passive */
            .end()
    }
}
