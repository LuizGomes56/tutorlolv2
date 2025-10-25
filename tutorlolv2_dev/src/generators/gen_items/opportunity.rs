use super::*;

impl Generator<ItemData> for Opportunity {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
