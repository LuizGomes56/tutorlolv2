use super::*;

impl Generator for SunfireAegis {
    fn generate(&mut self) -> MayFail {
        // Immolate damage per second. The other scaling is its radius
        self.damage_type(Magic).min_with(Passive, [0])?.end()
    }
}
