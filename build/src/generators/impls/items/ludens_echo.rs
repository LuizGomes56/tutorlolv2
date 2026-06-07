use super::*;

impl Generator for LudensEcho {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
