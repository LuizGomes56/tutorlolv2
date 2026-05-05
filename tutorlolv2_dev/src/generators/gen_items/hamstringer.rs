use super::*;

impl Generator for Hamstringer {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
