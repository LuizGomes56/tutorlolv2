use super::*;

impl Generator<ItemData> for OblivionOrb {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
