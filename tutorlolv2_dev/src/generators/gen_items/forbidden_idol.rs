use super::*;

impl Generator<ItemData> for ForbiddenIdol {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
