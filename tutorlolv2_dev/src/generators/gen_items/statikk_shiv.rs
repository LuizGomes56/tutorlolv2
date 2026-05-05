use super::*;

impl Generator for StatikkShiv {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
