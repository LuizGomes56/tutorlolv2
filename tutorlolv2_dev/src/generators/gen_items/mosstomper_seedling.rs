use super::*;

impl Generator for MosstomperSeedling {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
