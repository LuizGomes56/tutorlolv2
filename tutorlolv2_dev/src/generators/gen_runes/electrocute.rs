use super::*;

impl Generator for Electrocute {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Passive */
            .min(1)? /* Passive [1] */
            .min(2)? /* Passive [2] */
            .min(3)? /* Variable Damage */
            .damage_type(Physical)
            .damage_type(Physical)
            .end()
    }
}
