use super::*;

impl Generator<ItemData> for LudensEcho {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let passive = self.passive(0)?;
        let numbers = passive.capture_numbers::<f64>();
        let base_dmg = numbers.get(0).ok_or("Unable to extract base damage")?;
        let scaling = passive
            .split_once(" + ")
            .ok_or("Unable to extract scaling")?
            .1
            .trim();

        let min_dmg = format!("{base_dmg} + {scaling}");
        let max_dmg = format!("2 * ({min_dmg})");

        self.const_min_dmg(min_dmg);
        self.const_max_dmg(max_dmg);
        self.attr(Area);
        self.damage_type(Magic);
        self.end()
    }
}
