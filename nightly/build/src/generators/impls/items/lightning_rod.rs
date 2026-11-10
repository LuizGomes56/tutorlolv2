use super::*;

impl Generator for LightningRod {
    fn generate(&mut self) -> MayFail {
        // Call Lightning. The other scalings are the damage of a lightning that
        // strikes you
        self.damage_type(Magic).min_with(Passive, [0, 1, 2])?.end()
    }
}
