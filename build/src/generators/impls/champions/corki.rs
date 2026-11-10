use super::*;

impl Generator for Corki {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Ability {
                name: "Hextech Munitions".into(),
                damage_type: True,
                attributes: Undefined,
                comment: "Bonus true damage per basic attack".into(),
                damage: f![0.2 * AttackDamage],
            },
        )
        .ability(Key::Q, [("Magic Damage", Void)])
        .ability(
            Key::W,
            [
                ("Magic Damage Per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(
            Key::E,
            [
                ("Physical Damage Per Tick", Min),
                ("Total Physical Damage", Max),
            ],
        )
        .damage_types([E(Min), E(Max)], Physical)?
        .ability(
            Key::R,
            [
                ("Physical Damage", Min),
                ("Big One Physical Damage", Max),
            ],
        )
        .combos([
            &[
                Ability(W(Max)),
                Ability(E(Max)),
                Attack,
                Ability(Q(Void)),
                Ability(R(Min)),
                Ability(W(Max)),
                Attack,
                Ability(R(Min)),
                Attack,
            ],
            &[Ability(Q(Void)), Ability(R(Max)), Attack],
            &[
                Ability(Q(Void)),
                Ability(W(Max)),
                Ability(E(Max)),
                Attack,
                Attack,
            ],
            &[
                Ability(E(Max)),
                Ability(Q(Void)),
                Ability(R(Min)),
                Attack,
                Attack,
                Ability(R(Min)),
                Attack,
                Attack,
                Ability(R(Min)),
            ],
            &[
                Ability(W(Max)),
                Ability(E(Max)),
                Attack,
                Ability(R(Min)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Ability(R(Min)),
                Attack,
            ],
            &[
                Ability(E(Max)),
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ],
            &[Ability(Q(Void)), Ability(E(Max)), Attack, Attack],
        ])?
        .end()
    }
}
