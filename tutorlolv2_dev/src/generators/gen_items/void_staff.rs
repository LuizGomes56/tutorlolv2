use super::*;

impl Generator<ItemData> for VoidStaff {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
