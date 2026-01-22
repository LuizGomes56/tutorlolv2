use super::*;

impl Generator<ItemData> for OracleLens {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
