use super::*;

impl Generator for TearOfTheGoddess {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
