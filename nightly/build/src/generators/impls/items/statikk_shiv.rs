use super::*;

impl Generator for StatikkShiv {
    fn generate(&mut self) -> MayFail {
        // Electrospark: chain lightning of the Energized attack
        self.damage_type(Magic).attr(OnhitMax).min(Passive3)?.end()
    }
}
