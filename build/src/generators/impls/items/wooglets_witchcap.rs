use super::*;

impl Generator for WoogletsWitchcap {
    fn generate(&mut self) -> MayFail {
        // Magical Opus
        let [magical_opus] = self
            .description(Passive)?
            .capture(regex!(r"^Increase your \{\{as\|ability power\}\} by (\d+)%\.$"))?;

        self.constant("MAGICAL_OPUS_AP_AMPLIFIER", magical_opus / 100.0)?;

        Ok(())
    }
}
