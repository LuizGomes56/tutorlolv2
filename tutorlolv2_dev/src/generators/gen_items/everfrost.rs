use super::*;

impl Generator for Everfrost {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
