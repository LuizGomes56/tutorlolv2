use super::*;

impl Generator for CatalystOfAeons {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
