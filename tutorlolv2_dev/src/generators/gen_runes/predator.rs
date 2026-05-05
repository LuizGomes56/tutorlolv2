use super::*;

impl Generator for Predator {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Active - Predator */
            .min(1)? /* Description 2 */
            .min(2)? /* Description 2 [1] */
            .min(3)? /* Description 2 [2] */
            .min(4)? /* Description 3 */
            .min(5)? /* Passive */
            .damage_type(Physical)
            .damage_type(Physical)
            .end()
    }
}
