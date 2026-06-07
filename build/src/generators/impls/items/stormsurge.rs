use super::*;

impl Generator for Stormsurge {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
