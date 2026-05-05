use super::*;

impl Generator for KaenicRookern {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
