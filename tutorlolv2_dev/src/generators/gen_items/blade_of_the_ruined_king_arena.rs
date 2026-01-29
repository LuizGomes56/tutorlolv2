use super::*;

// #![stable]

impl Generator<ItemData> for BladeOfTheRuinedKingArena {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let bork = ItemFactory::run(ItemId::BladeOfTheRuinedKing)?;
        self.current_data.melee = bork.current_data.melee;
        self.current_data.ranged = bork.current_data.ranged;
        self.damage_type(bork.current_data.damage_type);
        self.infer_stats_ifdef();
        self.end()
    }
}
