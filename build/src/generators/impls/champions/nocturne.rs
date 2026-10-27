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
        .ability(Key::Q, [(2, Void) /* Physical damage */])
        .ability(
            Key::E,
            [
                (1, Min), /* Magic Damage per Tick */
                (2, Max), /* Total Magic Damage */
            ],
        )
        .ability(Key::R, [(0, Void) /* Physical Damage */])
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
