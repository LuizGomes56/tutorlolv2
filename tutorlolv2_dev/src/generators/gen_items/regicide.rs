use super::*;

impl Generator for Regicide {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
