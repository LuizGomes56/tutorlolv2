use super::*;

impl Generator for JakShoTheProtean {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
