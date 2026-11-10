use super::*;

impl Generator for DarksteelTalons {
    fn generate(&mut self) -> MayFail {
        // Gash: bonus true damage on-hit
        self.damage_type(True).attr(Onhit).min(Passive)?.end()
    }
}
