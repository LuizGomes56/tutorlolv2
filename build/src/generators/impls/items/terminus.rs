use super::*;

impl Generator for Terminus {
    fn generate(&mut self) -> MayFail {
        // Shadow
        self.damage_type(Magic).attr(Onhit).min(Passive)?.end()
    }
}
