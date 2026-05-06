use super::*;

impl Generator for ProwlersClaw {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Active)?.end()
    }
}
