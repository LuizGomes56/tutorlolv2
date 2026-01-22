use super::*;

impl Generator<ItemData> for GamblersBlade {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
