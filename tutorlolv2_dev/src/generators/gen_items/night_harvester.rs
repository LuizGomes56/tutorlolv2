use super::*;

impl Generator for NightHarvester {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
