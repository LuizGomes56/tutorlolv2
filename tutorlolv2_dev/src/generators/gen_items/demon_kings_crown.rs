use super::*;

impl Generator for DemonKingsCrown {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
