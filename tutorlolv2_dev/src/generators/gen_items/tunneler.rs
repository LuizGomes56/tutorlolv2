use super::*;

impl Generator<ItemData> for Tunneler {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
