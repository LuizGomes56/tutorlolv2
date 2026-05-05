use super::*;

impl Generator for AnvilVoucher {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
