use super::*;

impl Generator for TurboChemtank {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
