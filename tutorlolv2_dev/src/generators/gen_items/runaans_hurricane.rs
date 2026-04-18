use super::*;

impl Generator for RunaansHurricane {
    fn generate(&mut self) -> MayFail {
        let damage = self.passive(0)?;
        self.ranged_min_dmg(damage).attr(Area).damage_type(Physical).end()
    }
}
