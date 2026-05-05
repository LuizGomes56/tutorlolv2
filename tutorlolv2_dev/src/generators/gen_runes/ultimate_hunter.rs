use super::*;

impl Generator for UltimateHunter {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Description 0 */
            .min(1)? /* Description 1 */
            .end()
    }
}
