use super::*;

impl Generator for Mordekaiser {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Darkness Rise [1]", Void)])
            .modify(P(Void), |_| {
                f![5 + 0.3 * AbilityPower + (0.01 + 0.04 / 17 * (Level - 1)) * EnemyMaxHealth]
            })?
            .comment(P(Void), "Darkness Rise magic damage per second")?
            .ability(Key::Q, [("Magic Damage", Min)])
            .clone_with(Q(Min), Q(Max), |q| f![(q) * (1.3 + 0.05 * (QLevel - 1))])?
            .comment(Q(Max), "Isolated target")?
            .ability(Key::E, [("Magic Damage", Void)])
            .combos([
                &[Ability(E(Void)), Ability(Q(Max)), Attack, Attack],
                &[Ability(Q(Max)), Attack, Attack, Ability(E(Void)), Attack],
                &[Ability(E(Void)), Attack, Ability(Q(Max)), Attack],
                &[Attack, Ability(Q(Max)), Attack],
                &[Ability(E(Void)), Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
