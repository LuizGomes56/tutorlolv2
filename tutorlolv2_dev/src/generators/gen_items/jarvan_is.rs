use super::*;

impl Generator<ItemData> for JarvanIs {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
