use super::*;

impl Generator for PerfectTiming {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Passive */
            .end()
    }
}
