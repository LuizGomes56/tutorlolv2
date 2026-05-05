use super::*;

impl Generator for OhmwreckerTurretItem {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
