use super::*;

impl Generator for MejaisSoulstealer {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
