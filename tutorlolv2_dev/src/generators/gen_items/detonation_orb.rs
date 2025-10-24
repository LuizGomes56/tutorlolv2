use super::*;

impl Generator<ItemData> for DetonationOrb {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
