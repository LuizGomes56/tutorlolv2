use super::*;

impl Generator for LudensEcho {
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
