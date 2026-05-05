use super::*;

impl Generator for ExecutionersCalling {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(Physical).end()
    }
}
