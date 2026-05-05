use super::*;

impl Generator for PlatedSteelcaps {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
