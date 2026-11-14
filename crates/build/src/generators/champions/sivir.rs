use super::*;

impl Generator for Sivir {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Void))?
            .take("Minimum Damage", Q(Min))?
            .take("Total Maximum Champion Damage", Q(Max))?
            .take("Bounce Damage", W(Min))?
            .take("Bounce Critical Damage", W(Max))?;

        // Arena's blade scales with total AD instead of bonus AD: 40% of it at least,
        // and twice it on the way back
        let (mode, ad) = self.map_value(Key::Q, "AD ratio")?;

        for (id, share) in [(Q(Void), 1.0), (Q(Min), 0.4), (Q(Max), 2.0)] {
            let total = Formula::Const(0.0).with_map(mode, ad.clone().times(share));

            self.map_ratio(id, mode, &[BonusAd], 0.0)?
                .edit(id, |q| f![q + total * AttackDamage]);
        }

        self.combos([
            &[Ability(Q(Void)), Attack, Ability(W(Max)), Attack, Attack],
            &[Attack, Ability(W(Max)), Ability(Q(Void)), Attack, Attack],
            &[
                Attack,
                Ability(W(Max)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
            ],
            &[Attack, Ability(W(Max)), Ability(Q(Void))],
            &[Attack, Ability(Q(Void)), Attack],
        ])?
        .end()
    }
}
