use super::*;

impl Generator<ItemData> for KrakenSlayerArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
