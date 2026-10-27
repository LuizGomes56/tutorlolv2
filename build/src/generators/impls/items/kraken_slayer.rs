use super::*;

impl Generator for KrakenSlayer {
    fn generate(&mut self) -> MayFail {
        // Bring It Down, increased by up to 75% against targets with missing health
        self.damage_type(Physical).attr(OnhitMax).min(Passive)?;

        for attack_type in [Melee, Ranged] {
            let damage = self.formula_for(Passive, attack_type)?;
            self.assign(attack_type, Max, f![1.75 * damage]);
        }

        self.end()
    }
}
