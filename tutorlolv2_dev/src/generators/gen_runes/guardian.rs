use super::*;

impl Generator for Guardian {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Description 1 */
            .min(1)? /* Description 1 [1] */
            .min(2)? /* Description 2 */
            .min(3)? /* Passive */
            .end()
    }
}
