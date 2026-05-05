use super::*;

impl Generator for FleetFootwork {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Description 1 */
            .min(1) /* Unique – Energized */
            .end()
    }
}
