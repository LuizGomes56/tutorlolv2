use super::*;

impl Generator for SummonAery {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Adaptive Damage */
            .min(1) /* Description 1 */
            .min(2) /* Description 2 */
            .min(3) /* Passive */
            .min(4) /* Passive [1] */
            .min(5) /* Passive [2] */
            .min(6) /* Passive [3] */
            .damage_type(Physical)
            .end()
    }
}
