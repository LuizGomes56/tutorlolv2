use super::*;

impl Generator for ArdentCenser {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
