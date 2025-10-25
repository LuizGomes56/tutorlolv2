use super::*;

impl Generator<ItemData> for Trailblazer {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
