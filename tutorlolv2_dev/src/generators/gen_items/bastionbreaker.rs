use super::*;

impl Generator<ItemData> for Bastionbreaker {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let [melee, ranged] = match self.passive(1)?.capture_numbers::<f64>()[..] {
            [melee_base_dmg, _, melee_scaling] => {
                let melee_dmg = format!(
                    "{melee_base_dmg} + {scaling}",
                    scaling = format_args!("{melee_scaling} * {ArmorPenetrationFlat}")
                );
                let ranged_dmg = melee_dmg.mul(0.5);
                [melee_dmg, ranged_dmg]
            }
            _ => Err("Expected exactly 3 numbers in passive")?,
        };
        self.melee_min_dmg(melee);
        self.ranged_min_dmg(ranged);
        self.damage_type(True);
        self.end()
    }
}
