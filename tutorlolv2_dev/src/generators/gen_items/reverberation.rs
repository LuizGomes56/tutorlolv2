use super::*;

impl Generator<ItemData> for Reverberation {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
