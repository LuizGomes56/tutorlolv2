use super::*;

impl Generator<ItemData> for LegendaryFighterItemArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
