use super::*;

impl Generator for CosmicInsight {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Passive */
            .end()
    }
}
