use super::*;

impl Generator<ItemData> for Bandlepipes {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
