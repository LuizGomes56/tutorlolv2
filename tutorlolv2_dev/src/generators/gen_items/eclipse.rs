use super::*;

impl Generator for Eclipse {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
