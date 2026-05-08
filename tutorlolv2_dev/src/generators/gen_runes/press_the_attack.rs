use super::*;

impl Generator for PressTheAttack {
    fn generate(&mut self) -> MayFail {
        let formula = self.use_formula(1)?;
        self.assign_min(formula).damage_type(Adaptive).end()
    }
}
