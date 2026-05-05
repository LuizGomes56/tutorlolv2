use super::*;

impl Generator for HailOfBlades {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Description 1 */
            .min(1)? /* Description 1 [1] */
            .min(2)? /* Description 1 [2] */
            .min(3)? /* Passive */
            .damage_type(True)
            .end()
    }
}
