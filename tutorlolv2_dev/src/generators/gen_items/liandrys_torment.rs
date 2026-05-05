use super::*;

impl Generator for LiandrysTorment {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
