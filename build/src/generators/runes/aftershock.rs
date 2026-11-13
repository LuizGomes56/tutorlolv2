use super::*;

impl Generator for Aftershock {
    fn generate(&mut self) -> MayFail {
        for attack in [Melee, Ranged] {
            let damage = self.native("description2", attack, SourceForm::Base)?;
            self.assign(attack, Min, &damage);
        }

        self.damage_type(Magic).end()
    }
}
