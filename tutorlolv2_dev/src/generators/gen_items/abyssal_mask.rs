use super::*;

impl Generator for AbyssalMask {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
