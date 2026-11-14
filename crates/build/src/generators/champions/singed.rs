use super::*;

impl Generator for Singed {
    fn generate(&mut self) -> MayFail {
        self.take("Minimum Magic Damage", Q(Void))?
            .take("Magic Damage per Second", Q(Min))?
            .take("Magic Damage per Tick", Q(Max))?
            .take("Magic Damage", E(Void))?
            .combos([
                &[Ability(Q(Void)), Attack, Ability(E(Void)), Attack],
                &[Ability(Q(Void)), Ability(E(Void)), Attack, Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack, Attack],
                &[Ability(Q(Void)), Attack, Ability(E(Void)), Attack, Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
