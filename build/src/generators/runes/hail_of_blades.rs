use super::*;

impl Generator for HailOfBlades {
    fn generate(&mut self) -> MayFail {
        for attack in [Melee, Ranged] {
            let damage = self.native("description2", attack, SourceForm::Base)?;
            self.assign(attack, Min, &damage);
        }

        self.damage_type(True).end()
    }
}
