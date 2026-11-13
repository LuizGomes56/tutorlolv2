use super::*;

impl Generator for NashorsTooth {
    fn generate(&mut self) -> MayFail {
        self.attr(Onhit)
            .min(Named("Icathian Bite"))?
            .damage_type(Magic)
            .end()
    }
}
