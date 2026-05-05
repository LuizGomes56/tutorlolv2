use super::*;

impl Generator for ForceOfNature {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
