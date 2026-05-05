use super::*;

impl Generator for Fulmination {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
