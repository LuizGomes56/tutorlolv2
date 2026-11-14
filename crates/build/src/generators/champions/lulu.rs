use super::*;

impl Generator for Lulu {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 1, P(Void))?
            .comment(P(Void), "Pix's 3 bolts total bonus magic damage")
            .take("Magic Damage", Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Reduced Damage", Q(_1))?;

        // "Secondary bolt damage changed to 75% of primary", from 50%, by the position
        // of its numbers
        let (mode, numbers) = self.map_numbers(Key::Q, "Secondary bolt damage")?;
        let [share, ..] = numbers[..] else {
            return Err("[Lulu] Q: no number in \"Secondary bolt damage\"".into());
        };

        let secondary = Formula::Const(0.5).with_map(mode, Formula::Const(share / 100.0));
        let primary = self.damage(Q(Min))?;

        self.set(Q(_1), f![secondary * primary])
            .set(Q(Max), f![(1 + secondary) * primary])
            .take("Magic Damage", E(Void))?
            .combos([
                &[Attack, Ability(E(Void)), Attack, Ability(Q(Min)), Attack],
                &[Ability(E(Void)), Ability(Q(Min))],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(Q(Min)), Attack, Ability(E(Void))],
                &[Ability(E(Void)), Attack, Ability(Q(Min))],
                &[Ability(E(Void)), Ability(Q(Min)), Attack, Attack],
                &[Ability(Q(Min)), Ability(E(Void)), Attack],
                &[Ability(E(Void)), Attack, Ability(Q(Min)), Attack],
            ])?
            .end()
    }
}
