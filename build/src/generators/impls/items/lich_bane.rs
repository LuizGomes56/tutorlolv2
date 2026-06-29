use super::*;

impl Generator for LichBane {
    fn generate(&mut self) -> MayFail {
        let dmg = self
            .scaling(Passive, [0, 2])?
            .replace(AttackDamage.as_var(), BaseAd.as_var());

        self.damage_type(Magic).set_min(dmg).end()
    }
}
