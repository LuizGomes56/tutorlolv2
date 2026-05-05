use super::*;

impl Generator for TreasureHunter {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Description 0 */
            .min(1)? /* Description 1 */
            .min(2)? /* Description 2 */
            .end()
    }
}
