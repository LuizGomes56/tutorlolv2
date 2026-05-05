use super::*;

impl Generator for WhisperingCirclet {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
