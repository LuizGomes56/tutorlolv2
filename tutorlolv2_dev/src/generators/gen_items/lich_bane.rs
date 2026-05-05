use super::*;

impl Generator for LichBane {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
