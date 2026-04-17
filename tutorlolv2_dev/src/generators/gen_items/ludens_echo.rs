use super::*;

impl Generator<ItemData> for LudensEcho {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let passive = self.passive(0)?;
        let numbers = passive.capture_numbers::<f64>();
        let base_dmg = numbers.get(0).ok_or("Unable to extract base damage")?;
        let scaling = passive
            .split(" + ")
            .last()
            .ok_or("Unable to extract scaling")?
            .trim()
            .trim_end_matches("))");

        let min = base_dmg.plus(scaling);
        let max = min.parens().times(2);

        self.const_dmg(min, max).attr(Area).damage_type(Magic);
        self.end()
    }
}
