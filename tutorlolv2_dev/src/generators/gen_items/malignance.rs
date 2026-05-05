use super::*;

impl Generator for Malignance {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
