use super::*;

impl Generator<ItemData> for HauntingGuise {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
