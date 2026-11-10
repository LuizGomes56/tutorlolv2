use super::*;

impl Generator for ShieldBash {
    fn generate(&mut self) -> MayFail {
        self.min(1)?.damage_type(Adaptive).end()
    }
}
