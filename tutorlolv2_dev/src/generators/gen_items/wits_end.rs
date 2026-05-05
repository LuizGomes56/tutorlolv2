use super::*;

impl Generator for WitsEnd {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
