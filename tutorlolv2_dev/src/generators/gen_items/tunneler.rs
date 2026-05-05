use super::*;

impl Generator for Tunneler {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
