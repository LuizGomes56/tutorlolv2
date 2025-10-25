use super::*;

impl Generator<ItemData> for ControlWard {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
