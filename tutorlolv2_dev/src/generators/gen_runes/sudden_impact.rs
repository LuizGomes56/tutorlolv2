use super::*;

impl Generator for SuddenImpact {
    fn generate(&mut self) -> MayFail {
        self.min(0)?.damage_type(True).end()
    }
}
