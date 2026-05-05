use super::*;

impl Generator for CheapShot {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Description 2 */
            .min(1) /* Passive */
            .min(2) /* Valid Crowd Control */
            .damage_type(True)
            .end()
    }
}
