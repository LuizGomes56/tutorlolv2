use super::*;

impl Generator for Yorick {
    fn generate(&mut self) -> MayFail {
        self.take("Bonus Physical Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .insert_pet(P(Void), "Mist Walker")?
            .insert_pet(R(Void), "Maiden of the Mist")?
            .combos([
                &[Attack, Ability(E(Void)), Attack, Ability(Q(Void))],
                &[Ability(E(Void)), Attack, Ability(Q(Void)), Attack],
                &[Attack, Ability(Q(Void)), Ability(E(Void))],
            ])?
            .end()
    }
}
