use super::*;

impl Generator for PressTheAttack {
    fn generate(&mut self) -> MayFail {
        for attack_type in [Melee, Ranged] {
            let formula = self.use_formula_for(1, attack_type)?;
            self.assign(attack_type, Min, formula);
        }

        self.damage_type(Adaptive).end()
    }
}
