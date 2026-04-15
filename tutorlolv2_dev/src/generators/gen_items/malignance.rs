use super::*;

impl Generator<ItemData> for Malignance {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        const TICKS: usize = (/* Duration */3.0 / 0.25/* Frequency */) as _;

        let damage_per_tick = format!("15 + 0.0125 * {AbilityPower}");

        self.const_min_dmg(&damage_per_tick);
        self.const_max_dmg(format!("{TICKS} * ({damage_per_tick})"));
        self.damage_type(Magic);
        self.attr(Area);
        self.end()
    }
}
