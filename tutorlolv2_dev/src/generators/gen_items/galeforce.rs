use super::*;

impl Generator for Galeforce {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
