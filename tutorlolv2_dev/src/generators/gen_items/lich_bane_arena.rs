use super::*;

impl Generator<ItemData> for LichBaneArena {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.yield_to(ItemId::LichBaneArena)?;
        self.end()
    }
}
