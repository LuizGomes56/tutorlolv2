use super::*;

impl Generator for DarksteelTalons {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(True).end()
    }
}
