use super::*;

impl Generator for SuddenImpact {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Passive */
            .damage_type(True)
            .end()
    }
}
