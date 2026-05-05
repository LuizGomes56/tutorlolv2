use super::*;

impl Generator for HealthPotion {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
