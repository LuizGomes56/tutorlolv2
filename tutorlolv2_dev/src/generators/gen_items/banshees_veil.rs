use super::*;

impl Generator for BansheesVeil {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
