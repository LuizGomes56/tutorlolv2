use super::*;

impl GeneratorV2 for Senna {
    fn generate(&mut self) -> MayFail {
        self.take("Innate - Relic Cannon", P(_1))?
            .take("Innate - Weakened Soul", P(Void))?
            .set(P(_1), f![0.2 * AttackDamage])
            .take("Physical Damage", Q(Void))?
            .take("Physical Damage", W(Void))?
            .take("Physical Damage", R(Void))?
            .damage_type(P(Void), Physical)
            .damage_type(P(_1), Physical)
            .damage_type(R(Void), Physical)
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(R(Void)), Ability(Q(Void))],
                &[Ability(W(Void)), Ability(Q(Void)), Ability(R(Void))],
                &[Ability(Q(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
