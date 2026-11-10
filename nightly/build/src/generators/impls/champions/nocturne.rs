use super::*;

impl Generator for Nocturne {
    fn generate(&mut self) -> MayFail {
        self.insert(
            P(Void),
            Ability {
                name: "Umbra Blades".into(),
                damage_type: Physical,
                attributes: Undefined,
                comment: "Empowered basic attack".into(),
                damage: f![1.2 * AttackDamage],
            },
        )
        .ability(Key::Q, [("Physical damage", Void)])
        .ability(
            Key::E,
            [
                ("Magic Damage per Tick", Min),
                ("Total Magic Damage", Max),
            ],
        )
        .ability(Key::R, [("Physical Damage", Void)])
        .combos([
            &[
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
                Attack,
            ],
            &[Ability(Q(Void)), Ability(E(Max)), Attack, Attack, Attack],
            &[
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
                Attack,
                Attack,
                Attack,
            ],
            &[
                Ability(R(Void)),
                Ability(Q(Void)),
                Ability(R(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
                Attack,
            ],
            &[
                Ability(Q(Void)),
                Ability(R(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[
                Ability(Q(Void)),
                Ability(E(Max)),
                Attack,
                Ability(R(Void)),
                Attack,
                Ability(R(Void)),
                Attack,
                Attack,
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
            &[
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(E(Max)),
                Attack,
                Attack,
                Attack,
                Ability(Q(Void)),
                Attack,
                Attack,
                Attack,
            ],
        ])?
        .end()
    }
}
