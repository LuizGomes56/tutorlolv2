use super::*;

impl Generator<ItemData> for ExecutionersCalling {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
