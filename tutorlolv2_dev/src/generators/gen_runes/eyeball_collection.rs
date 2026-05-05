use super::*;

impl Generator for EyeballCollection {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Adaptive */
            .min(1) /* Description 1 */
            .min(2) /* Passive */
            .end()
    }
}
