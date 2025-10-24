use super::*;

impl Generator<ItemData> for RaiseMorale {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
