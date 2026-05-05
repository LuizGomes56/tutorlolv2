use super::*;

impl Generator for UnsealedSpellbook {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* 6:00 */
            .min(1)? /* Description 2 */
            .min(2)? /* Passive */
            .end()
    }
}
