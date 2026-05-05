use super::*;

impl Generator for Stormsurge {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
