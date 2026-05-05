use super::*;

impl Generator for HextechFlashtraption {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Active */
            .min(1) /* Description 3 */
            .min(2) /* Passive */
            .min(3) /* Recast */
            .end()
    }
}
