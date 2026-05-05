use super::*;

impl Generator for CloakOfStarryNight {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
