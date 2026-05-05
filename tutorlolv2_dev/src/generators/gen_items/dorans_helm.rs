use super::*;

impl Generator for DoransHelm {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(Physical).end()
    }
}
