use super::*;

impl Generator<ItemData> for ArcaneSweeper {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
