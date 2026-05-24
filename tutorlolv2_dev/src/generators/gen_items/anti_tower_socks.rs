use super::*;

impl Generator for AntiTowerSocks {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
