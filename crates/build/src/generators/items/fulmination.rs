use super::*;

impl Generator for Fulmination {
    fn generate(&mut self) -> MayFail {
        // Dynamo: Energized attack
        self.damage_type(Magic)
            .trigger(Trigger::Energized)
            .min(Named("Dynamo"))?
            .end()
    }
}
