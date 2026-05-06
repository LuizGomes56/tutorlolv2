use super::*;

impl Generator for BamisCinder {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Passive)?.end()
    }
}
