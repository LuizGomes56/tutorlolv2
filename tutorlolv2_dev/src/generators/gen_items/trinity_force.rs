use super::*;

impl Generator for TrinityForce {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Passive)?.end()
    }
}
