use super::*;

impl Generator for PhantomDancer {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
