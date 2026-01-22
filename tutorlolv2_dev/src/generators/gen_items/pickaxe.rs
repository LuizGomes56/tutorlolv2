use super::*;

impl Generator<ItemData> for Pickaxe {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
