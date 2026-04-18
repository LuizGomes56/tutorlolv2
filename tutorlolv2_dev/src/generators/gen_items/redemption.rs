use super::*;

impl Generator for Redemption {
    fn generate(&mut self) -> MayFail {
        let damage = self.active(0)?;
        self.const_min_dmg(damage).attr(Area).damage_type(True).end()
    }
}
