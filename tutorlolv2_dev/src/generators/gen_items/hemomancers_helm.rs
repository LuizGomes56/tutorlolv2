use super::*;

impl Generator for HemomancersHelm {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(True).end()
    }
}
