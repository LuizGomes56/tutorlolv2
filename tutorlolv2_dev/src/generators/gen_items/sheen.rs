use super::*;

impl Generator for Sheen {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Passive)?.end()
    }
}
