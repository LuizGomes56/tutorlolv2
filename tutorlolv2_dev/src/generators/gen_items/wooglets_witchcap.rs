use super::*;

impl Generator for WoogletsWitchcap {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.min(Passive)?.end()
    }
}
