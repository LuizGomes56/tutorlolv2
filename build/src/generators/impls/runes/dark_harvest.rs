use super::*;

impl Generator for DarkHarvest {
    fn generate(&mut self) -> MayFail {
        for attack_type in [Melee, Ranged] {
            let scalings = self.scaling_for(2, 0..3, attack_type)?;
            self.assign(attack_type, Min, f![30 + scalings]);
        }

        self.damage_type(Adaptive).end()
    }
}
