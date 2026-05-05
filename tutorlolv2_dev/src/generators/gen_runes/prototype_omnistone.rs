use super::*;

impl Generator for PrototypeOmnistone {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Applicable Keystones */
            .min(1) /* Description 2 */
            .min(2) /* Passive */
            .end()
    }
}
