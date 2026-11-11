use super::*;

impl Generator for Hubris {
    fn generate(&mut self) -> MayFail {
        // Eminence
        let [melee_ad, ranged_ad, melee_ad_per_stack, ranged_ad_per_stack] = self
            .description(Passive)?
            .capture(regex!(r"\{\{as\|\{\{rd\|(\d+)\|(\d+)\}\}\|ad\}\} \(\+ \{\{rd\|(\d+)\|(\d+)\}\} per stack\)"))?;

        self.constant("EMINENCE_MELEE_AD", melee_ad)?
            .constant("EMINENCE_RANGED_AD", ranged_ad)?
            .constant("EMINENCE_MELEE_AD_PER_STACK", melee_ad_per_stack)?
            .constant("EMINENCE_RANGED_AD_PER_STACK", ranged_ad_per_stack)?;

        // Eminence on the modes that change it, by attack type
        let eminence = |item: &Self, pattern| -> MayFail<Vec<(MapMode, [f64; 2])>> {
            item.map_texts(pattern)?
                .into_iter()
                .map(|(mode, text)| {
                    let values = text.capture(regex!(r"\{\{rd\|(\d+)\|(\d+)\}\}"))?;
                    Ok((mode, values))
                })
                .collect()
        };

        let ad = eminence(self, regex!(r"^Eminence base bonus attack damage changed to"))?;
        let per_stack = eminence(self, regex!(r"^Eminence bonus attack damage per stack changed to"))?;

        self.map_constant("EMINENCE_MELEE_AD", ad.iter().map(|(m, [melee, _])| (*m, *melee)))?
            .map_constant("EMINENCE_RANGED_AD", ad.iter().map(|(m, [_, ranged])| (*m, *ranged)))?
            .map_constant(
                "EMINENCE_MELEE_AD_PER_STACK",
                per_stack.iter().map(|(m, [melee, _])| (*m, *melee)),
            )?
            .map_constant(
                "EMINENCE_RANGED_AD_PER_STACK",
                per_stack.iter().map(|(m, [_, ranged])| (*m, *ranged)),
            )?;

        Ok(())
    }
}
