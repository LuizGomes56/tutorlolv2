use super::*;

impl Generator<ItemData> for Decapitator {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
