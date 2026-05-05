use super::*;

impl Generator for ScoutingAhead {
    fn generate(&mut self) -> MayFail {
        self.min(Active).damage_type(True).end()
    }
}
