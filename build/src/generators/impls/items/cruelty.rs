use super::*;

impl Generator for Cruelty {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        let scalings = self.scaling(Passive, 0..2)?;
        self.damage_type(Magic)
            /* Missing base damage (based on level) */
            .asgn_min(scalings)
            .end()
    }
}
