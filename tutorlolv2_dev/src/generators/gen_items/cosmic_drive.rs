use super::*;

impl Generator<ItemData> for CosmicDrive {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
