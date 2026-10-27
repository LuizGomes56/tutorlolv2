use super::*;

impl Generator for ProwlersClaw {
    fn generate(&mut self) -> MayFail {
        // Sandswipe
        self.damage_type(Physical).min(Active)?.end()
    }
}
