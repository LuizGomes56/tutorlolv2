use super::*;

impl Generator for Stormrazor {
    fn generate(&mut self) -> MayFail {
        // Bolt: Energized attack. The scaling is the movement speed it grants
        self.damage_type(Magic)
            .trigger(Trigger::Energized)
            .min(Named("Bolt"))?
            .end()
    }
}
