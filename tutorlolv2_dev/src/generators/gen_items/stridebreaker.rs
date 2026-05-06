use super::*;

impl Generator for Stridebreaker {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Active)?.end()
    }
}
