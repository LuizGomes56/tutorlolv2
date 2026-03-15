use super::*;

impl Generator<ItemData> for MalignanceArena {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.yield_to(ItemId::Malignance)?;
        self.end()
    }
}
