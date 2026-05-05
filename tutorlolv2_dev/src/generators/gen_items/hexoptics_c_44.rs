use super::*;

impl Generator for HexopticsC44 {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
