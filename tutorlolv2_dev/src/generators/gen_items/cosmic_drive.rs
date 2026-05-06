use super::*;

impl Generator for CosmicDrive {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Passive)?.end()
    }
}
