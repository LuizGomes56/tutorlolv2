use super::*;

impl Generator<ItemData> for GargoyleStoneplate {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
