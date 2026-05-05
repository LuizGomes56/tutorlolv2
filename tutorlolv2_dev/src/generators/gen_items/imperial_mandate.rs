use super::*;

impl Generator for ImperialMandate {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
