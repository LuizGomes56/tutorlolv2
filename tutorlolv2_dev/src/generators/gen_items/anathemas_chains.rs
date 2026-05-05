use super::*;

impl Generator for AnathemasChains {
    fn generate(&mut self) -> MayFail {
        self.min(Active).min(Passive).end()
    }
}
