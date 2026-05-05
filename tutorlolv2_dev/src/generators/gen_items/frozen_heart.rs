use super::*;

impl Generator for FrozenHeart {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
