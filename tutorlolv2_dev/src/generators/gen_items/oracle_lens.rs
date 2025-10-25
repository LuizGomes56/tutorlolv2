use super::*;

impl Generator<ItemData> for OracleLens {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
