use super::*;

impl GeneratorV2 for Braum {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take_nth("Innate", 2, P(_1))?
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage", R(Void))?;

        // Bonus magic damage equal to 40% of the Concussive Blows damage
        let concussive_blows = self.damage(P(Void))?;

        self.set(P(_1), f![0.4 * (concussive_blows)])
            .combos([
                &[Ability(Q(Void)), Ability(R(Void))],
                &[Ability(Q(Void)), Attack, Attack, Attack],
                &[Ability(R(Void)), Ability(Q(Void)), Attack, Attack, Attack],
                &[Ability(Q(Void)), Attack, Ability(R(Void)), Attack, Attack],
                &[Attack, Ability(Q(Void)), Attack, Attack],
            ])?
            .end()
    }
}
