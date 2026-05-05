use super::*;

impl Generator for RadiantVirtue {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
