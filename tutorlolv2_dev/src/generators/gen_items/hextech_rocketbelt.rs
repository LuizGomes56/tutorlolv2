use super::*;

impl Generator for HextechRocketbelt {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Active)?.end()
    }
}
