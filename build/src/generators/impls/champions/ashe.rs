use super::*;

impl Generator for Ashe {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Ability {
                name: "Frost Shot".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Bonus damage per basic attack, based on critical strike chance".into(),
                damage: f![AttackDamage * CritChance / 100],
            },
        )
        .ability(
            Key::Q,
            [
                ("Physical Damage Per Arrow", Min),
                ("Total Damage Per Flurry", Max),
            ],
        )
        .ability(Key::W, [("Physical Damage", Void)])
        .ability(Key::R, [("Magic Damage", Void)])
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
