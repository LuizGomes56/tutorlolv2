use super::*;

impl Generator<ItemData> for WorldAtlas {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
