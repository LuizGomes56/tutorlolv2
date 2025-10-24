use super::*;

impl Generator<ItemData> for MercurialScimitar {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
