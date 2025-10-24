use super::*;

impl Generator<ItemData> for WardensMail {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
