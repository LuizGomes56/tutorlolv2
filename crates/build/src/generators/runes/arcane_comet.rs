use super::*;

impl Generator for ArcaneComet {
    fn generate(&mut self) -> MayFail {
        for attack in [Melee, Ranged] {
            let damage = self.native("description", attack, SourceForm::Base)?;
            self.assign(attack, Min, &damage);

            let maximum = self.native("description", attack, SourceForm::Maximum)?;
            self.assign(attack, Max, maximum);
        }

        self.damage_type(Adaptive).end()
    }
}
