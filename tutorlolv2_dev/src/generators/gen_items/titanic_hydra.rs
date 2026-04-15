use super::*;

impl Generator<ItemData> for TitanicHydra {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let [melee, ranged] = self
            .active(0)?
            .capture_numbers::<f64>()
            .get(0..2)
            .ok_or("Failed to get melee and ranged scalings")?
            .into_iter()
            .map(|v| format!("{v} * {MaxHealth}"))
            .collect::<Vec<_>>()
            .try_into()
            .map_err(vec_err::<2, _>)?;

        self.melee_max_dmg(&melee);
        self.ranged_max_dmg(&ranged);
        self.melee_min_dmg(melee.mul(0.25));
        self.ranged_min_dmg(ranged.mul(0.25));
        self.attr(AreaOnhit);
        self.damage_type(Physical);
        self.end()
    }
}
