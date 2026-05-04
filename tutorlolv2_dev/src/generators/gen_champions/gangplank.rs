use super::*;

impl Generator for Gangplank {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, _1) /* Physical Damage */]).ability_nth(1, Key::Q, [(1, _2) /* Physical Damage */]).ability(Key::E, [(0, _1) /* Bonus Champion Damage */]).ability(Key::R, [(0, _1) /* Magic Damage Per Cluster */, (1, _2) /* Magic Damage Per Wave */, (2, _3) /* Maximum Mixed Total Damage with  Fire at Will and  Death's Daughter */, (3, _4) /* Total Magic Damage */, (4, _5) /* Total Magic Damage with  Fire at Will */, (5, _6) /* Total Mixed Damage with  Death's Daughter */, (6, _7) /* True Damage with  Death's Daughter */]).end()
    }
}
