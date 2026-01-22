use super::*;

impl Generator<ItemData> for SeryldasGrudge {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
