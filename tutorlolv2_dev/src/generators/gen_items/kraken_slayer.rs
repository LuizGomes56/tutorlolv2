use super::*;

impl Generator<ItemData> for KrakenSlayer {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let melee_dmg = format!("150.0 + 5.0 * ({Level} - 8.0).max(0.0)");
        let ranged_dmg = format!("120.0 + 4.0 * ({Level} - 8.0).max(0.0)");
        let max = |dmg| format!("(1.0 + 0.75 * {EnemyMissingHealth}) * ({dmg})");

        self.melee_min_dmg(&melee_dmg);
        self.ranged_min_dmg(&ranged_dmg);
        self.melee_max_dmg(max(melee_dmg));
        self.ranged_max_dmg(max(ranged_dmg));
        self.attr(OnhitMax);
        self.damage_type(Physical);
        self.end()
    }
}
