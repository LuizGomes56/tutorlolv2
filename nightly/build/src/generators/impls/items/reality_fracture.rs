use super::*;

impl Generator for RealityFracture {
    fn generate(&mut self) -> MayFail {
        // ZZ'Rot: damage of each Voidmite attack
        self.damage_type(Magic).min(Passive)?.end()
    }
}
