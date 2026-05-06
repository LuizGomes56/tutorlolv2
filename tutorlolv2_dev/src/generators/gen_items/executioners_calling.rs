use super::*;

impl Generator for ExecutionersCalling {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Passive)?.end()
    }
}
