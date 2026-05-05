use super::*;

impl Generator for LegendTenacity {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Legend */
            .min(1)? /* Passive */
            .end()
    }
}
