use super::*;

impl Generator for CheapShot {
    fn generate(&mut self) -> MayFail {
        for attack in [Melee, Ranged] {
            let damage = self.native("description", attack, SourceForm::Base)?;
            self.assign(attack, Min, &damage);
        }

        self.damage_type(True).end()
    }
}
