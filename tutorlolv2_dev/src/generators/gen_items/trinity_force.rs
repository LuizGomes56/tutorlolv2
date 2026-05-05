use super::*;

impl Generator for TrinityForce {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).damage_type(Physical).end()
    }
}
