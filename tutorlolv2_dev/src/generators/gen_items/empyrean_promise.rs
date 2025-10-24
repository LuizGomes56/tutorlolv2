use super::*;

impl Generator<ItemData> for EmpyreanPromise {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
