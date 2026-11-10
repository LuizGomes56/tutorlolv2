use super::*;

impl Generator for HexboltCompanion {
    fn generate(&mut self) -> MayFail {
        // Covering Fire
        self.damage_type(Physical).min(Passive)?.end()
    }
}
