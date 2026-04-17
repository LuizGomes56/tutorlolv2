use super::*;

impl Generator<ItemData> for KrakenSlayer {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let bound = Level.minus(8).parens() + ".max(0)";
        let melee_dmg = 150.plus(5).times(&bound);
        let ranged_dmg = 120.plus(4).times(bound);
        let max = |dmg: String| {
            1.plus(0.75)
                .times(EnemyMissingHealth)
                .parens()
                .times(dmg.parens())
        };

        self.melee_min_dmg(&melee_dmg)
            .ranged_min_dmg(&ranged_dmg)
            .melee_max_dmg(max(melee_dmg))
            .ranged_max_dmg(max(ranged_dmg))
            .nonstandard()
            .attr(OnhitMax)
            .damage_type(Physical);

        self.end()
    }
}
