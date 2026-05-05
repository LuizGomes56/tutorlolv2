use super::*;

impl Generator for DeathsDaughter {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
