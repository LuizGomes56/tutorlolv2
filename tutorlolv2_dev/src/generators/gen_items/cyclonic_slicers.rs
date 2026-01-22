use super::*;

impl Generator<ItemData> for CyclonicSlicers {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
