use super::*;

impl Generator<ItemData> for NashorsTooth {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self
            .meraki_data
            .passives
            .get(0)
            .ok_or(format!(
                "[{name}]: meraki_data.{{field}}[0] does not exist",
                name = self.meraki_data.name,
            ))?
            .effects
            .get_damage();

        self.current_data.melee.minimum_damage = damage.clone();
        self.current_data.ranged.minimum_damage = damage;

        self.current_data.attributes = Attrs::Onhit;
        self.current_data.damage_type = DamageType::Magic;

        Ok(self.0)
    }
}
