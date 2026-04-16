use super::*;

impl Generator<ItemData> for Eclipse {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let [melee, ranged] = self.capture(Passive(0), |v| format!("{v} * {EnemyMaxHealth}"))?;

        self.melee_min_dmg(melee);
        self.ranged_min_dmg(ranged);
        self.damage_type(Magic);
        self.end()
    }
}
