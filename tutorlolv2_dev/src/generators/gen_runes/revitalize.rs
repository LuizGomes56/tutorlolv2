use super::*;

impl Generator for Revitalize {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Description 1 */
            .min(1) /* Description 2 */
            .min(2) /* Passive */
            .end()
    }
}
