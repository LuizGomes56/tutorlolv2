use super::*;

impl GeneratorV2 for Ashe {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Damage {
                name: "Frost Shot".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Bonus damage per basic attack, based on critical strike chance".into(),
                formula: f![AttackDamage * CritChance / 100],
            },
        )?
        .take("Physical Damage Per Arrow", Q(Min))?
        .take("Total Damage Per Flurry", Q(Max))?
        .take("Physical Damage", W(Void))?
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
