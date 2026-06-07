use super::*;

impl Generator for BrambleVest {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Magic).min(Passive)?.end()
    }
}
