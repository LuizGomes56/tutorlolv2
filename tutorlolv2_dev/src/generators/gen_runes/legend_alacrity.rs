use super::*;

impl Generator for LegendAlacrity {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Legend */
            .min(1)? /* Passive */
            .end()
    }
}
