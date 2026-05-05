use super::*;

impl Generator for WardensEye {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(True).end()
    }
}
