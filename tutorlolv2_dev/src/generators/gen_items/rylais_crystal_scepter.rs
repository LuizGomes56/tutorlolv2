use super::*;

impl Generator for RylaisCrystalScepter {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
