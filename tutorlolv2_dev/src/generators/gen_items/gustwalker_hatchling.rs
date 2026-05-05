use super::*;

impl Generator for GustwalkerHatchling {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
