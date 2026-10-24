use super::*;

impl Generator for NashorsTooth {
    fn generate(&mut self) -> MayFail {
        self.attr(Onhit).min(Passive)?.damage_type(Magic).end()
    }
}
