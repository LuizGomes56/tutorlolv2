use super::*;

impl Generator for GuinsoosRageblade {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
