use super::*;

impl Generator<ItemData> for AtmasReckoningArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
