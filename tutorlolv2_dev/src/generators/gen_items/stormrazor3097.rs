use super::*;

impl Generator for Stormrazor3097 {
    fn generate(&mut self) -> MayFail {
        let passive = self.passive(1)?;
        let damage = passive
            .split_once(" + ")
            .ok_or("Failed to ignore move speed scaling")?
            .0;
        self.const_min_dmg(damage).attr(OnhitMax).damage_type(Magic).end()
    }
}
