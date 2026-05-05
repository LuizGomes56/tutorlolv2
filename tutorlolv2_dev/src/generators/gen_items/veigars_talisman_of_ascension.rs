use super::*;

impl Generator for VeigarsTalismanOfAscension {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
