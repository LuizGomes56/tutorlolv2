use super::*;

impl Generator for HextechRocketbelt {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.damage_type(True).end()
    }
}
