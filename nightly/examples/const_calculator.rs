#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::num::NonZeroU8;
use tutorlolv2::{calculator::*, model::*, *};

const GAME: InputGame<'static> = InputGame {
    active_player: InputActivePlayer {
        runes: &[RuneId::Electrocute],
        rune_exceptions: &[],
        abilities: AbilityLevels {
            q: 5,
            w: 5,
            e: 5,
            r: 3,
        },
        current_health: 0.0,
        data: InputMinData {
            champion_id: ChampionId::Neeko,
            items: &[ItemId::NashorsTooth, ItemId::RabadonsDeathcap],
            item_exceptions: &[],
            stats: None,
            stacks: 0,
            level: NonZeroU8::new(18).unwrap(),
            is_mega_gnar: false,
        },
    },
    enemy_players: &[InputMinData {
        champion_id: ChampionId::Aatrox,
        items: &[ItemId::ForceOfNature],
        item_exceptions: &[],
        stats: None,
        stacks: 0,
        level: NonZeroU8::new(18).unwrap(),
        is_mega_gnar: false,
    }],
    dragons: Dragons::default(),
    game_map: GameMap::SummonersRift,
};

const LAYOUT: DamageLayout = calculator_layout(GAME);
const OUTPUT: ArrayOutputGame<LAYOUT, { GAME.enemy_players.len() }> =
    calculator_array::<LAYOUT, { GAME.enemy_players.len() }>(GAME);

fn main() {
    // The macro infers both dimensions and forces compile-time evaluation.
    let inferred = tutorlolv2::calculator_array!(GAME);
    let boxed = calculator(GAME);
    for row in 0..OUTPUT.damages.rows() {
        assert_eq!(OUTPUT.damages.row(row), boxed.damages.row(row));
        assert_eq!(inferred.damages.row(row), boxed.damages.row(row));
    }
    println!("Layout: {LAYOUT:?}");
    println!("Rows: {}", OUTPUT.damages.rows());
    println!("Enemy damage: {:?}", OUTPUT.enemy_damages(0));
    println!(
        "Callable sizes: formula={} bytes, batch={} bytes",
        size_of::<Closure>(),
        size_of::<AbilityBatch>()
    );
}
