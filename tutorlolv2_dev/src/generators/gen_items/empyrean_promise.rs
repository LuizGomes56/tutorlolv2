use super::*;

impl Generator for EmpyreanPromise {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
