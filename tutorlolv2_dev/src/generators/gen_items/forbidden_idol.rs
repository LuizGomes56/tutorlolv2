use super::*;

impl Generator<ItemData> for ForbiddenIdol {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
