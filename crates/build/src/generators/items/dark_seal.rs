use super::*;

impl Generator for DarkSeal {
    fn generate(&mut self) -> MayFail {
        // Glory
        let [glory] = self.description(Named("Glory"))?.capture(regex!(
            r"For every stack, gain \{\{as\|(\d+) ability power\}\}"
        ))?;

        self.constant("GLORY_AP_PER_STACK", glory)?;

        Ok(())
    }
}
