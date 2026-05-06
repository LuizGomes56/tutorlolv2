use super::*;

impl Generator for SunfireAegis {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).min(Passive)?.end()
    }
}
