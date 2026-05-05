use super::*;

impl Generator for CoupDeGrace {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Passive */
            .end()
    }
}
