use super::*;

impl Generator for ReapersToll {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
