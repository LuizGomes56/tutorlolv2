use super::*;

impl Generator for WintersApproach {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
