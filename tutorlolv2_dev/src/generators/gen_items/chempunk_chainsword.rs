use super::*;

impl Generator for ChempunkChainsword {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(Physical).end()
    }
}
