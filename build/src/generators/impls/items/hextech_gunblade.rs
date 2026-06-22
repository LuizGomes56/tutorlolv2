use super::*;

impl Generator for HextechGunblade {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Magic).end()
    }
}
