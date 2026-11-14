use super::*;

impl Generator for Talon {
    fn generate(&mut self) -> MayFail {
        // The daggers thrown out, before they return
        let initial = self.value_nth("Physical Damage", 0, Key::W)?;
        let initial = self.map_changed(Key::W, "Initial base damage", initial)?;
        let initial = self.map_changed(Key::W, "Outgoing bonus AD ratio", initial)?;

        self.take_nth("Innate", 0, P(Void))?
            .map_change(P(Void), "Total bonus AD ratio")?
            .take("Physical Damage", Q(Min))?
            .take("Critical Physical Damage", Q(Max))?
            .take_nth("Physical Damage", 1, W(Min))?
            .map_change(W(Min), "Return base damage")?
            .map_change(W(Min), "Incoming bonus AD ratio")?
            .take("Total Physical Damage", W(Max))?;

        let back = self.damage(W(Min))?;

        self.set(W(Max), f![initial + back])
            .take("Physical Damage", R(Min))?
            .take("Total Physical Damage", R(Max))?
            .combos([
                &[Ability(W(Max)), Ability(Q(Max)), Ability(R(Max)), Attack],
                &[
                    Ability(W(Max)),
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Max)), Ability(R(Max)), Ability(Q(Max)), Attack],
                &[Ability(W(Max)), Attack, Ability(Q(Max)), Attack],
                &[Ability(W(Max)), Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
