use super::*;

impl Generator<ItemData> for StaffOfFlowingWater {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
