use core::num::NonZeroU8;
use tutorlolv2::{model::*, *};

#[test]
fn prepared_views_borrow_nested_collections_without_layout_casts() {
    let request = OwnedInputGame {
        active_player: OwnedInputActivePlayer {
            runes: vec![RuneId::Electrocute].into_boxed_slice(),
            data: OwnedInputMinData {
                items: vec![ItemId::NashorsTooth].into_boxed_slice(),
                ..Default::default()
            },
            ..Default::default()
        },
        enemy_players: vec![OwnedInputMinData {
            champion_id: ChampionId::Aatrox,
            items: vec![ItemId::FrozenHeart].into_boxed_slice(),
            ..Default::default()
        }]
        .into_boxed_slice(),
        dragons: Dragons::default(),
        game_map: GameMap::Unknown,
    };
    let prepared = request.prepare();
    let input = prepared.as_input();
    assert_eq!(input.enemy_players.len(), 1);
    assert!(core::ptr::eq(
        input.active_player.runes.as_ptr(),
        request.active_player.runes.as_ptr()
    ));
    assert!(core::ptr::eq(
        input.enemy_players[0].items.as_ptr(),
        request.enemy_players[0].items.as_ptr()
    ));
    assert_eq!(input.enemy_players[0].champion_id, ChampionId::Aatrox);
}

#[cfg(feature = "engine")]
#[test]
fn decoded_owned_request_is_usable_by_calculator() {
    let request = OwnedInputGame {
        enemy_players: vec![OwnedInputMinData {
            champion_id: ChampionId::Aatrox,
            level: NonZeroU8::new(18).unwrap(),
            ..Default::default()
        }]
        .into_boxed_slice(),
        ..Default::default()
    };
    let bytes = bincode::encode_to_vec(&request, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    let (decoded, consumed): (OwnedInputGame, _) =
        bincode::decode_from_slice(&bytes, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(consumed, bytes.len());
    let prepared = decoded.prepare();
    let output = crate::calculator::calculator(prepared.as_input());
    assert_eq!(output.enemies.len(), 1);
    assert_eq!(output.enemies[0].champion_id, ChampionId::Aatrox);
    assert_eq!(output.damages.rows(), crate::L_MSTR + 1);
    assert_eq!(
        output.enemy_damages(0).abilities.len(),
        decoded.active_player.data.champion_id.number_of_abilities()
    );

    // Owned requests and the table-backed response both cross the transport
    // boundary without retaining any borrowed views in the encoded data.
    let bytes = bincode::encode_to_vec(&output, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    let (roundtrip, consumed): (OutputGame, _) =
        bincode::decode_from_slice(&bytes, tutorlolv2_types::BINCODE_CONFIG).unwrap();
    assert_eq!(consumed, bytes.len());
    assert_eq!(roundtrip, output);
    assert_eq!(roundtrip.enemy_damages(0), output.enemy_damages(0));
    for m in 0..crate::L_MSTR {
        assert_eq!(roundtrip.monster_damages(m), output.monster_damages(m));
    }
}
