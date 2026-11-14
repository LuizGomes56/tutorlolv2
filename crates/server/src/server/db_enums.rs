//! Closed value sets that only the server stores (spec §15.2, D17). The ones
//! shared with the desktop live in `protocol`.

/// Postgres type `riot_game_source`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "riot_game_source")]
pub enum RiotGameSource {
    Lcu,
    Spectator,
}

/// Postgres type `share_delete_reason`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "share_delete_reason")]
pub enum ShareDeleteReason {
    GameEnded,
    Revoked,
}

/// Postgres type `session_kind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, sqlx::Type)]
#[sqlx(type_name = "session_kind")]
pub enum SessionKind {
    Web,
    Desktop,
}
