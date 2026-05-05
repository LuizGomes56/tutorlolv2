use super::*;

impl Generator for LethalTempo {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Adaptive Damage */
            .min(1) /* Description 1 */
            .min(2) /* Description 2 */
            .min(3) /* Passive */
            .damage_type(Physical)
            .end()
    }
}
