use super::*;

impl Generator<ItemData> for KircheisShard {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.const_min_dmg(50).attr(OnhitMax).damage_type(Magic);
        self.end()
    }
}
