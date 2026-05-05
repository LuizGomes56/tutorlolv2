use super::*;

impl Generator for HollowRadiance {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
