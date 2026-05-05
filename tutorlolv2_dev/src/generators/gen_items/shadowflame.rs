use super::*;

impl Generator for Shadowflame {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(True).end()
    }
}
