use super::*;

impl Generator for GuardianAngel {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
