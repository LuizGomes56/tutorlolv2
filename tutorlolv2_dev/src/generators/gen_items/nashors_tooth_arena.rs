use super::*;

// #![stable]

impl Generator<ItemData> for NashorsToothArena {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let nashor = ItemFactory::run(ItemId::NashorsTooth)?;
        self.current_data.melee = nashor.current_data.melee;
        self.current_data.ranged = nashor.current_data.ranged;
        self.damage_type(nashor.current_data.damage_type);
        // self.stats = self.infer_stats();
        self.end()
    }
}
