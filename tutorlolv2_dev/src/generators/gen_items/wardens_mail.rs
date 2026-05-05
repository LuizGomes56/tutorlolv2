use super::*;

impl Generator for WardensMail {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
