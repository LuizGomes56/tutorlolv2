use super::*;

impl Generator for BlackCleaver {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).damage_type(Physical).end()
    }
}
