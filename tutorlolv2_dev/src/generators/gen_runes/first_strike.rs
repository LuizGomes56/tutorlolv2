use super::*;

impl Generator for FirstStrike {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Description 1 */
            .min(1)? /* Passive */
            .damage_type(True)
            .end()
    }
}
