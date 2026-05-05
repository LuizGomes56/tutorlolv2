use super::*;

impl Generator for ProwlersClaw {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.damage_type(Physical).end()
    }
}
