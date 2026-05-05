use super::*;

impl Generator for Heartsteel {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
