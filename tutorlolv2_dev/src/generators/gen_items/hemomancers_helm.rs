use super::*;

impl Generator for HemomancersHelm {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Passive)?.end()
    }
}
