use super::*;

impl Generator for NavoriFlickerblade {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
