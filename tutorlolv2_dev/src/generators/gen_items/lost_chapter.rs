use super::*;

impl Generator<ItemData> for LostChapter {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
