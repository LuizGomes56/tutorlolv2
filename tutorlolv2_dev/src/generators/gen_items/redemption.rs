use super::*;

impl Generator for Redemption {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
