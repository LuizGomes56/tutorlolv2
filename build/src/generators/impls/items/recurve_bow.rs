use super::*;

impl Generator for RecurveBow {
    fn generate(&mut self) -> MayFail {
        // Sting
        self.damage_type(Physical).attr(Onhit).min(Passive)?.end()
    }
}
