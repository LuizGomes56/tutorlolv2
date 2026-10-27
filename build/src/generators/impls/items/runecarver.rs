use super::*;

impl Generator for Runecarver {
    fn generate(&mut self) -> MayFail {
        // Spiral Out: damage of each missile
        self.damage_type(Magic).min(Passive3)?.end()
    }
}
