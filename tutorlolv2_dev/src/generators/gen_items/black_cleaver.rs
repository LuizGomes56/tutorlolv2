use super::*;

impl Generator for BlackCleaver {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Passive)?.end()
    }
}
