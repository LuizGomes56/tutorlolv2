use super::*;

impl Generator<Champion> for Jinx {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::W, [(0, 0, Void)]);
        self.ability(Key::E, [(0, 0, Void)]);
        self.ability(
            Key::R,
            [
                // Surroundings
                (1, 0, _1Max),
                (1, 1, _1Min),
                // Primary Target
                (2, 0, Max),
                (2, 1, Min),
            ],
        );

        let q = self
            .data
            .abilities
            .q
            .first()
            .ok_or("Failed to find Jinx's Q")?
            // n = 1 generates unrecognized pattern
            .format(vec![1.1.times(AttackDamage); 5]);

        self.insert(Q(Void), q);

        self.attr(AreaOnhitMax, [Q(Void)])?;
        self.attr(Area, [Q(Void), E(Void), R(_1Max), R(_1Min), R(Max), R(Min)])?;
        self.combo([
            Ability(E(Void)),
            Attack,
            Ability(R(Min)),
            Attack,
            Ability(W(Void)),
        ])?;

        self.progress(Stable);
        self.end()
    }
}
