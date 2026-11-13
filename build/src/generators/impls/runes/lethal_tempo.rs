use super::*;

impl Generator for LethalTempo {
    fn generate(&mut self) -> MayFail {
        let melee = self.use_formula_for(1, Melee)?;
        let ranged = self.use_formula_for(1, Ranged)?;

        self.assign(Melee, Min, melee)
            .assign(Ranged, Min, f![(ranged) * 6 / 9])
            .damage_type(Adaptive)
            .end()
    }
}
