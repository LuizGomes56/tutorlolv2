use super::*;

impl Generator for WarmogsArmor {
    fn generate(&mut self) -> MayFail {
        // Warmog's Vitality, which only scales the health granted by items
        let vitality = self.effect(Passive3)?.scalings[0]
            .try_get_value()
            .ok_or("Expected a constant stat scaling")?;

        self.description(Passive3)?
            .capture::<0>(regex!(r"bonus''' health '''from items'''"))?;

        self.constant("WARMOGS_VITALITY_BONUS_HEALTH_RATIO", vitality)?;

        Ok(())
    }
}
