use super::*;

impl Generator for TheCollector {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
