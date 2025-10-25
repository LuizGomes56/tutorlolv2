use super::*;

impl Generator<ItemData> for AmplifyingTome {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
