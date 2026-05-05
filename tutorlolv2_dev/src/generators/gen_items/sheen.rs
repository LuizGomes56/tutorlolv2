use super::*;

impl Generator for Sheen {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).damage_type(Physical).end()
    }
}
