use super::*;

impl Generator for LethalTempo {
    fn generate(&mut self) -> MayFail {
        let formula = self.use_formula(1)?;
        self.assign(Melee, Min, &formula)
            .assign(Ranged, Min, f![(formula) * 6 / 9])
            .damage_type(Adaptive)
            .end()
    }
}
