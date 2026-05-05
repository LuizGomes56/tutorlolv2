use super::*;

impl Generator for KinkouJitte {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
