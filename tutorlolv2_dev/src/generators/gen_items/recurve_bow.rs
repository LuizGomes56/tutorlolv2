use super::*;

impl Generator for RecurveBow {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Passive)?.end()
    }
}
