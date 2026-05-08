use super::*;

impl Generator for DeathfireTouch {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical)
            .min(0)? /* Adaptive Damage */
            .min(1)? /* Description 0 */
            .min(2)? /* Description 0 [1] */
            .min(3)? /* Description 0 [2] */
            .min(4)? /* Description 0 [3] */
            .min(5)? /* Description 0 [4] */
            .min(6)? /* Description 0 [5] */
            .min(7)? /* Description 1 */
            .end()
    }
}
