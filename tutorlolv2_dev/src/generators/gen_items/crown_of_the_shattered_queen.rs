use super::*;

impl Generator for CrownOfTheShatteredQueen {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
