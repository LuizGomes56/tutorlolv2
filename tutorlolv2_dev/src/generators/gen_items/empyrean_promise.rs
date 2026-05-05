use super::*;

impl Generator for EmpyreanPromise {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
