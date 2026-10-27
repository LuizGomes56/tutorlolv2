use super::*;

impl Generator for HellfireHatchet {
    fn generate(&mut self) -> MayFail {
        // Char: burn over 4 seconds, at the maximum health difference of 3000
        self.damage_type(Physical).min(Passive)?.end()
    }
}
