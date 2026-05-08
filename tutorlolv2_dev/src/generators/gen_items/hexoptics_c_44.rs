use super::*;

impl Generator for HexopticsC44 {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
