use super::*;

impl Generator for Rageknife {
    fn generate(&mut self) -> MayFail {
        self.const_min_dmg(20).attr(Onhit).damage_type(Magic).end()
    }
}
