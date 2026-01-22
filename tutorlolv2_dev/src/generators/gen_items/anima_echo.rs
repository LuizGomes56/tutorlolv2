use super::*;

impl Generator<ItemData> for AnimaEcho {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
