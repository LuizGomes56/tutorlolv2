use super::*;

impl Generator for TowerPowerUp {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
