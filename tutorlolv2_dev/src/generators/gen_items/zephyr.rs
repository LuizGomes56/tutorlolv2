use super::*;

impl Generator for Zephyr {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
