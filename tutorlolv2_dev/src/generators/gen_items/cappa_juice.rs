use super::*;

impl Generator<ItemData> for CappaJuice {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
