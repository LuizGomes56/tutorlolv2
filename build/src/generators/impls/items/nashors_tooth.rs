use super::*;

impl Generator for NashorsTooth {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(Magic).end()
    }
}
