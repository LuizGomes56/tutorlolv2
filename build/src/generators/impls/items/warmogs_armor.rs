use super::*;

impl Generator for WarmogsArmor {
    fn generate(&mut self) -> MayFail {
        // Warmog's Vitality, which only scales the health granted by items
        let [vitality] = self
            .formula(Passive3)?
            .capture(regex!(r"^\(\(([\d.]+)\) \* ctx\.bonus_health\)$"))?;

        self.description(Passive3)?
            .capture::<0>(regex!(r"bonus''' health '''from items'''"))?;

        self.constant("WARMOGS_VITALITY_BONUS_HEALTH_RATIO", vitality)?;

        Ok(())
    }
}
