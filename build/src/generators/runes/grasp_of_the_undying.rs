use super::*;

impl Generator for GraspOfTheUndying {
    fn generate(&mut self) -> MayFail {
        for attack in [Melee, Ranged] {
            let damage = self.native("description", attack, SourceForm::Base)?;
            self.assign(attack, Min, &damage);
        }

        self.damage_type(Magic).end()
    }
}
