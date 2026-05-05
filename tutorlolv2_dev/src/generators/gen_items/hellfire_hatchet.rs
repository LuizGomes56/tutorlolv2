use super::*;

impl Generator for HellfireHatchet {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
