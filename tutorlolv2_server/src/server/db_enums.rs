//! Closed value sets that only the server stores (spec §15.2, D17). The ones
//! shared with the desktop live in `tutorlolv2_protocol`.

/// Postgres type `riot_game_source`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "riot_game_source", rename_all = "snake_case")]
pub enum RiotGameSource {
    Lcu,
    Spectator,
}

/// Postgres type `share_delete_reason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "share_delete_reason", rename_all = "snake_case")]
pub enum ShareDeleteReason {
    GameEnded,
    Revoked,
}
