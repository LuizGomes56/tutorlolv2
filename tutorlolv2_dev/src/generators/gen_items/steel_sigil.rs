use super::*;

impl Generator<ItemData> for SteelSigil {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
