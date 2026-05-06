use super::*;

impl Generator for Tiamat {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Active)?.end()
    }
}
