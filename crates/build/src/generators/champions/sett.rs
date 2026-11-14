use super::*;

impl Generator for Sett {
    fn generate(&mut self) -> MayFail {
        // A value written in a shape map changes can't read, by the position of its
        // numbers, as {{fd|3}}% of "3% per 100 bonus AD"
        let (mode, numbers) = self.map_numbers(Key::W, "Damage base Grit ratio")?;
        let [grit, ..] = numbers[..] else {
            return Err("[Sett] W: no number in \"Damage base Grit ratio\"".into());
        };

        self.take("Innate - Heavy Hands", P(Void))?
            // The bonus of Knuckle Down's attacks, without the attack itself
            .take("Physical Damage Per Hit", Q(Min))?
            .edit(Q(Min), |q| f![q - AttackDamage])
            .take("Total Physical Damage", Q(Max))?
            .edit(Q(Max), |q| f![q - 2 * AttackDamage])
            // Haymaker by Grit, his mana: the actual damage with his current Grit, and its
            // range from none to resourceMax
            .take("Damage", W(_1Max))?
            .map_ratio(W(_1Max), mode, &[MaxMana], grit / 100.0)?
            .derive(W(Void), W(_1Max), |w| {
                w.replace_var(MaxMana, &Formula::Var(CurrentMana))
            })?
            .derive(W(_1Min), W(_1Max), |w| {
                w.replace_var(MaxMana, &Formula::Const(0.0))
            })?
            .take("Physical Damage", E(Void))?
            .take("Reduced Damage", R(Min))?
            .take("Physical Damage", R(Max))?
            .damage_type(P(Void), Physical)
            .damage_type(W(Void), Physical)
            .damage_type(W(_1Min), Physical)
            .damage_type(W(_1Max), Physical)
            .damage_type(R(Min), Physical)
            .damage_type(R(Max), Physical)
            .combos([
                &[Ability(Q(Max)), Ability(E(Void)), Ability(R(Max))],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[Attack, Attack, Ability(Q(Max)), Attack, Attack],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(W(Void)),
                ],
            ])?
            .end()
    }
}
