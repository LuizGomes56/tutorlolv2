use super::*;

impl Generator for IonianBootsOfLucidity {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
