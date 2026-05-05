use super::*;

impl Generator for CosmicDrive {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(True).end()
    }
}
