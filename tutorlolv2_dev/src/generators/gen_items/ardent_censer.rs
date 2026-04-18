use super::*;

impl Generator for ArdentCenser {
    fn generate(&mut self) -> MayFail {
        let passive = self.passive(0)?;
        let damage = passive
            .split_once(" + ")
            .ok_or("Failed to ignore attack speed scaling")?
            .1;

        self.const_min_dmg(damage).attr(OnhitMax).damage_type(Magic).end()
    }
}
