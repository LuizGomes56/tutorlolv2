use super::*;

impl Generator for CheapShot {
    fn generate(&mut self) -> MayFail {
        self.min(1)?.damage_type(True).end()
    }
}
