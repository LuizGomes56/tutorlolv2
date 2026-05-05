use super::*;

impl Generator for EssenceReaver {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
