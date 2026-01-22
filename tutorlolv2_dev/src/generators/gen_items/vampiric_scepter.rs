use super::*;

impl Generator<ItemData> for VampiricScepter {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
