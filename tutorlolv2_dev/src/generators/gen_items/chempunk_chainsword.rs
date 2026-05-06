use super::*;

impl Generator for ChempunkChainsword {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Passive)?.end()
    }
}
