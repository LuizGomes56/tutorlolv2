use super::*;

impl Generator for Hexdrinker {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
