use super::*;

impl Generator<ItemData> for TurboChemtank {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
