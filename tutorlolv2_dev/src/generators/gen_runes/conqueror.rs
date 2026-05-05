use super::*;

impl Generator for Conqueror {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Description 1 */
            .min(1) /* Description 1 [1] */
            .min(2) /* Description 1 [2] */
            .min(3) /* Description 1 [3] */
            .min(4) /* Description 2 */
            .min(5) /* Passive */
            .end()
    }
}
