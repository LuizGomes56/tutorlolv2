use super::*;

impl Generator for GamblersBlade {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
