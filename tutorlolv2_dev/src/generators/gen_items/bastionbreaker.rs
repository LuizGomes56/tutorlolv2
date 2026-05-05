use super::*;

impl Generator for Bastionbreaker {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
