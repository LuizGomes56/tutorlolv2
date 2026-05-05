use super::*;

impl Generator for Dawncore {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
