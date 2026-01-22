use super::*;

impl Generator<ItemData> for EmpyreanPromise {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
