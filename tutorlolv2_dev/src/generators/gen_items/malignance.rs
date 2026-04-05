use super::*;

impl Generator<ItemData> for Malignance {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        const DURATION: f64 = 3.0;
        const FREQUENCY: f64 = 0.25;
        const TICKS: usize = (DURATION / FREQUENCY) as _;

        let damage_per_tick = format!("15 + 0.0125 * {AbilityPower}");

        self.const_min_dmg(&damage_per_tick);
        self.const_max_dmg(format!("{TICKS} * ({damage_per_tick})"));
        self.damage_type(Magic);
        self.attr(Area);
        self.end()
    }

    fn progress(&self) -> Progress {
        Stable
    }
}
