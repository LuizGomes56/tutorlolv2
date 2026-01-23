use super::*;

// #![stable]

impl Generator<ItemData> for BladeOfTheRuinedKingArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        ItemFactory::run(ItemId::BladeOfTheRuinedKing)
    }
}
