use super::*;

impl Generator for DoransShield {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).damage_type(True).end()
    }
}
