use super::*;

impl Generator for HauntingGuise {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
