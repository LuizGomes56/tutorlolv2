use super::*;

impl Generator for DivineSunderer {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
