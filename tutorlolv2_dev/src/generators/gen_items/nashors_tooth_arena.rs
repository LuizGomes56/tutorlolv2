use super::*;

impl Generator<ItemData> for NashorsToothArena {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.yield_to(ItemId::NashorsTooth)?;
        self.end()
    }
}
