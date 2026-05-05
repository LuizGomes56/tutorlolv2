use super::*;

impl Generator for ImmortalShieldbow {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
