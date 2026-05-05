use super::*;

impl Generator for SpectralCutlass {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
