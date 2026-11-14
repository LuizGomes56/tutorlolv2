use super::*;

impl Generator for KrakenSlayer {
    fn generate(&mut self) -> MayFail {
        // Bring It Down, increased by up to 75% against targets with missing health
        self.damage_type(Physical)
            .attr(OnhitMax)
            .min(Event("Bring It Down", BaseProc))?;

        let amplification = self.maximum_amplification_formula(Named("Bring It Down"))?;
        let factor = f![1 + amplification];

        for attack_type in [Melee, Ranged] {
            let damage = self.formula_for(Event("Bring It Down", BaseProc), attack_type)?;
            self.assign(attack_type, Max, f![damage * factor]);
        }

        self.end()
    }
}
