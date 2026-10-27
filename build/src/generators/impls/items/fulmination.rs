use super::*;

impl Generator for Fulmination {
    fn generate(&mut self) -> MayFail {
        // Dynamo: Energized attack
        self.damage_type(Magic).attr(OnhitMax).min(Passive3)?.end()
    }
}
