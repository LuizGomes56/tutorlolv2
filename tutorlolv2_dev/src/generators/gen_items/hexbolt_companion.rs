use super::*;

impl Generator for HexboltCompanion {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
