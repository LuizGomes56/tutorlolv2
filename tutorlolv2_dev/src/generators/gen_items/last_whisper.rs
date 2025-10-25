use super::*;

impl Generator<ItemData> for LastWhisper {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
