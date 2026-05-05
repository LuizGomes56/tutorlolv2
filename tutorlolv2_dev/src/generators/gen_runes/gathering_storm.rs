use super::*;

impl Generator for GatheringStorm {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Adaptive */
            .min(1) /* Passive */
            .end()
    }
}
