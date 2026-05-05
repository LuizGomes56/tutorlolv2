use super::*;

impl Generator for TurretPlating {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
