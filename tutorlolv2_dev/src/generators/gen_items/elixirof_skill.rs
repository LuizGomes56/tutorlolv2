use super::*;

impl Generator<ItemData> for ElixirofSkill {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
