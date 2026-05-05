use super::*;

impl Generator for BladeOfTheRuinedKing {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
