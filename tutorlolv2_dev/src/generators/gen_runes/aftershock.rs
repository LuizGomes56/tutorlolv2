use super::*;

impl Generator for Aftershock {
    fn generate(&mut self) -> MayFail {
        self.min(0)?.damage_type(Magic).end()
    }
}
