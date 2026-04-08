use super::*;

impl Generator<ItemData> for BladeOfTheRuinedKingArena {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.yield_to(ItemId::BladeOfTheRuinedKing)?;
        self.end()
    }
}
