use super::*;

impl Generator for DeathsDance {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
