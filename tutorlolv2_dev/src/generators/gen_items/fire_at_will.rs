use super::*;

impl Generator for FireAtWill {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
