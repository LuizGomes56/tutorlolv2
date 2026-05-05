use super::*;

impl Generator for MercurialScimitar {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
