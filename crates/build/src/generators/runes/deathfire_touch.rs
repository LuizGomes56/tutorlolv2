use super::*;

impl Generator for DeathfireTouch {
    fn generate(&mut self) -> MayFail {
        let [increase] = self
            .description("description")?
            .capture(regex!(r"increasedby (\d+)%"))?;

        let multiplier = 1.0 + increase / 100.0;

        for attack in [Melee, Ranged] {
            let damage = self.native("description", attack, SourceForm::Base)?;

            self.assign(attack, Min, &damage);
            self.assign(attack, Max, f![damage * multiplier]);
        }

        self.damage_type(Magic).end()
    }
}
