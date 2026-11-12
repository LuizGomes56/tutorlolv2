use super::*;

impl GeneratorV2 for Mordekaiser {
    fn generate(&mut self) -> MayFail {
        let isolated = self.value("Isolated Damage Increase", Key::Q)?;

        self.take_nth("Darkness Rise", 0, P(Void))?
            .set(P(Void), {
                f![5 + 0.3 * AbilityPower + (0.01 + 0.04 / 17 * (Level - 1)) * EnemyMaxHealth]
            })
            .comment(P(Void), "Darkness Rise magic damage per second")
            .take("Magic Damage", Q(Min))?
            .derive(Q(Max), Q(Min), |q| f![q * (1 + isolated / 100)])?
            .comment(Q(Max), "Isolated target")
            .take("Magic Damage", E(Void))?
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
