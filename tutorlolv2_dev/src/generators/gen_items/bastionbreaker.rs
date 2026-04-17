use super::*;

impl Generator<ItemData> for Bastionbreaker {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let [melee, ranged] = match self.passive(1)?.capture_numbers::<f64>()[..] {
            [base_dmg, _, scaling] => {
                let melee_dmg = base_dmg.plus(scaling.times(ArmorPenetrationFlat));
                let ranged_dmg = melee_dmg.parens().times(0.5);
                [melee_dmg, ranged_dmg]
            }
            _ => Err("Expected exactly 3 numbers in passive")?,
        };

        self.melee_min_dmg(melee)
            .ranged_min_dmg(ranged)
            .damage_type(True);

        self.end()
    }
}
