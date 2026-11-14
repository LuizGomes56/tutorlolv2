use super::*;

impl Generator for JuiceOfPower {
    fn generate(&mut self) -> MayFail {
        // Grants one of them, depending on the adaptive type
        let [ad, ad_ratio, ap, ap_ratio] = self.description(Consume)?.capture(
            regex!(r"^Grants \{\{as\|(\d+)\|AD\}\} \{\{as\|\(\+ (\d+)% AD\)\}\} \{\{as\|'''bonus''' attack damage\}\} or \{\{as\|(\d+)\|AP\}\} \{\{as\|\(\+ (\d+)% AP\)\}\} \{\{as\|ability power\}\} \(\[\[Adaptive force\|Adaptive\]\]\)"),
        )?;

        self.constant("ATTACK_DAMAGE", ad)?
            .constant("ATTACK_DAMAGE_RATIO", ad_ratio / 100.0)?
            .constant("ABILITY_POWER", ap)?
            .constant("ABILITY_POWER_RATIO", ap_ratio / 100.0)?;

        Ok(())
    }
}
