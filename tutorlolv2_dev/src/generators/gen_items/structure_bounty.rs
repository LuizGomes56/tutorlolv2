use super::*;

impl Generator<ItemData> for StructureBounty {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
