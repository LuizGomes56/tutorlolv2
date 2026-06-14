use super::*;

impl Generator for Scorch {
    fn generate(&mut self) -> MayFail {
        self.min(0)?.damage_type(Magic).end()
    }
}
