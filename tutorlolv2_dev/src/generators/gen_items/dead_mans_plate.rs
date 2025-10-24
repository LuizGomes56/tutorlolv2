use super::*;

impl Generator<ItemData> for DeadMansPlate {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
