use super::*;

impl Generator for GuardiansDirk {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
