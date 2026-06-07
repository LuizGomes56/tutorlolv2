use super::*;

impl Generator for LethalTempo {
    fn generate(&mut self) -> MayFail {
        let formula = self.use_formula(1)?;
        self.assign(Melee, Min, &formula)
            .assign(Ranged, Min, formula.parenthesize().times(6.0 / 9.0))
            .damage_type(Adaptive)
            .end()
    }
}
