use super::*;

impl Generator for VoidStaff {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
