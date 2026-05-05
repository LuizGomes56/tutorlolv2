use super::*;

impl Generator for Overcharged {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
