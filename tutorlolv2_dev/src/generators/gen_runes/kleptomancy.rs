use super::*;

impl Generator for Kleptomancy {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Applicable Consumables */
            .min(1) /* Description 2 */
            .min(2) /* Passive */
            .end()
    }
}
