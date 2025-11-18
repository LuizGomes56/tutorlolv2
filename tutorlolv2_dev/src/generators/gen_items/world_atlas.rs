use super::*;

impl Generator<ItemData> for WorldAtlas {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
