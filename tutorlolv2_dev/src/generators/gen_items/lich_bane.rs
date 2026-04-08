use super::*;

impl Generator<ItemData> for LichBane {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let passive = self.passive(0)?;
        let base_ad_scaling = passive.capture_percent(1)? / 100.0;
        let ap_scaling = passive
            .split_once('+')
            .ok_or("Failed to capture ability_power scaling")?
            .1
            .trim();
        let damage = format!("{base_ad_scaling} * {BaseAd} + {ap_scaling}");
        self.const_min_dmg(damage);
        self.attr(OnhitMax);
        self.damage_type(Magic);
        self.end()
    }
}
