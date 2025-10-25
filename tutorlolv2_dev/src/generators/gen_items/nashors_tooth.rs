use super::*;

impl Generator<ItemData> for NashorsTooth {
    #[item_generator]
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self
            .meraki_data
            .passives
            .get(0)
            .ok_or("Nashor's Tooth: meraki_data.passives[0] does not exist")?
            .effects
            .get_damage();

        self.current_data.melee.minimum_damage = damage.clone();
        self.current_data.ranged.minimum_damage = damage;

        self.current_data.attributes = Attrs::Onhit;
        self.current_data.damage_type = DamageType::Magic;
    }
}
