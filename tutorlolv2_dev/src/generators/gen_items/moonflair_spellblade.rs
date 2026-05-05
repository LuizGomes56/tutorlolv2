use super::*;

impl Generator for MoonflairSpellblade {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
