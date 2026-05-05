use super::*;

impl Generator for DetonationOrb {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).damage_type(True).end()
    }
}
