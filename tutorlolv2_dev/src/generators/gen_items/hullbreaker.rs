use super::*;

impl Generator for Hullbreaker {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
