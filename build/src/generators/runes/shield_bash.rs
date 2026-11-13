use super::*;

impl Generator for ShieldBash {
    fn generate(&mut self) -> MayFail {
        // The generic Stacks input carries the triggering shield amount for this rune.
        for attack in [Melee, Ranged] {
            let damage = self.native("description", attack, SourceForm::Base)?;
            self.assign(attack, Min, &damage);
        }

        self.damage_type(Adaptive).end()
    }
}
