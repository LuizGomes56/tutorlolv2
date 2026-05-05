use super::*;

impl Generator for RecurveBow {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).damage_type(Physical).end()
    }
}
