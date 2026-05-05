use super::*;

impl Generator for Runecarver {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
