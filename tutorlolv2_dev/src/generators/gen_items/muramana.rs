use super::*;

impl Generator for Muramana {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
