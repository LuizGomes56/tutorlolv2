use super::*;

impl Generator for JuiceOfVitality {
    fn generate(&mut self) -> MayFail {
        let [health, max_health] = self.description(Consume)?.capture(regex!(
            r"^Grants \{\{as\|(\d+)\|hp\}\} \{\{as\|\(\+ (\d+)% '''maximum''' health\)\}\}"
        ))?;

        self.constant("HEALTH", health)?
            .constant("MAX_HEALTH_RATIO", max_health / 100.0)?;

        Ok(())
    }
}
