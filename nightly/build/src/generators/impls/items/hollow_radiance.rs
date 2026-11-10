use super::*;

impl Generator for HollowRadiance {
    fn generate(&mut self) -> MayFail {
        // Immolate damage per second, and the eruption of Desolate. The other
        // scalings are Immolate's radius and the eruption against champions
        self.damage_type(Magic)
            .min_with(Passive, [0])?
            .max_with(Passive2, [0])?
            .end()
    }
}
