use super::*;

impl Generator for AbsoluteFocus {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Adaptive */
            .min(1)? /* Passive */
            .min(2)? /* Passive [1] */
            .min(3)? /* Passive [2] */
            .end()
    }
}
