use super::*;

impl Generator for Ashe {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            // A share by critical strike chance of the attack's damage
            .edit(P(Void), |share| f![share * AttackDamage])
            .damage_type(P(Void), Physical)
            .comment(
                P(Void),
                "Bonus damage per basic attack, based on critical strike chance",
            )
            .take("Physical Damage Per Arrow", Q(Min))?
            .take("Total Damage Per Flurry", Q(Max))?
            .take("Physical Damage", W(Void))?
            .map_change(W(Void), "Base damage")?;

        // Arena's Volley scales with total AD instead of bonus AD
        let (mode, ad) = self.map_value(Key::W, "AD ratio")?;
        let mut volley = self.damage(W(Void))?.on(mode);
        let base = volley.base_mut().cloned().ok_or("[Ashe] W has no base")?;
        let arena = f![base + ad * AttackDamage];

        self.edit(W(Void), |w| {
            w.on(MapMode::Default).with_map(mode, arena.clone())
        })
        .take("Magic Damage", R(Void))?
        .combos([
            &[
                Attack,
                Ability(Q(Max)),
                Ability(R(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
            ],
            &[Ability(W(Void)), Attack],
            &[Ability(R(Void)), Attack],
            &[Attack, Attack, Attack, Attack, Ability(Q(Max))],
            &[
                Ability(R(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Max)),
                Attack,
            ],
            &[Attack, Ability(W(Void)), Attack],
        ])?
        .end()
    }
}
