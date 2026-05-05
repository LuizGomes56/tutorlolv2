use super::*;

impl Generator for OracleLens {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
