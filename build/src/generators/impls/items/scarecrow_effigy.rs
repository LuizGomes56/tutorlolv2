use super::*;

impl Generator for ScarecrowEffigy {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
