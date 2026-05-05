use super::*;

impl Generator for MortalReminder {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).damage_type(Physical).end()
    }
}
