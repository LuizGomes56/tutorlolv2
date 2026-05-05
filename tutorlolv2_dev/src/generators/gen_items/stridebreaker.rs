use super::*;

impl Generator for Stridebreaker {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.damage_type(Physical).end()
    }
}
