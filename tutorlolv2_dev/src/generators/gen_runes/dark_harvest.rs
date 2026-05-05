use super::*;

impl Generator for DarkHarvest {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Adaptive Damage */
            .min(1)? /* Description 1 */
            .min(2)? /* Passive */
            .damage_type(Physical)
            .end()
    }
}
