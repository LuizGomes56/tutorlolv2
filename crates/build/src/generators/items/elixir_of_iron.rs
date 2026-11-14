use super::*;

impl Generator for ElixirOfIron {
    fn generate(&mut self) -> MayFail {
        let [health] = self
            .description(Consume)?
            .capture(regex!(r"^Grants \{\{as\|(\d+) '''bonus''' health\}\}"))?;

        self.constant("BONUS_HEALTH", health)?;

        Ok(())
    }
}
