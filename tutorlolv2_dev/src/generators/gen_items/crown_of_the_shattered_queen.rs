use super::*;

impl Generator for CrownOfTheShatteredQueen {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
