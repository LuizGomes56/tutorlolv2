use super::*;

impl Generator for WorldAtlas {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
