use super::*;

impl Generator<ItemData> for LegendaryTankItemArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
