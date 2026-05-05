use super::*;

impl Generator for MinionDematerializer {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Description 1 */
            .min(1)? /* Passive */
            .end()
    }
}
