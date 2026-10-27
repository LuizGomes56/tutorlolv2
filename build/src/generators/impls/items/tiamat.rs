use super::*;

impl Generator for Tiamat {
    fn generate(&mut self) -> MayFail {
        // Crescent
        self.damage_type(Physical).min(Active)?.end()
    }
}
