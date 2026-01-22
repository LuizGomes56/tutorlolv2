use super::*;

impl Generator<ItemData> for TheBrutalizer {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
