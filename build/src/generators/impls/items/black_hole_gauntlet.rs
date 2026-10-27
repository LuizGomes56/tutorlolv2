use super::*;

impl Generator for BlackHoleGauntlet {
    fn generate(&mut self) -> MayFail {
        // Black hole damage per second. The other scaling is its duration
        self.damage_type(Magic).min_with(Active, [1])?.end()
    }
}
