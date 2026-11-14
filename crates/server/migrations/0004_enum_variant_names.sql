-- Store enum labels using the default Rust variant names. Existing rows and
-- constraints retain their enum identities when PostgreSQL renames the labels.

ALTER TYPE end_reason RENAME VALUE 'game_end_event' TO 'GameEndEvent';
ALTER TYPE end_reason RENAME VALUE 'lcu_end_of_game' TO 'LcuEndOfGame';
ALTER TYPE end_reason RENAME VALUE 'lcu_left_game' TO 'LcuLeftGame';
ALTER TYPE end_reason RENAME VALUE 'superseded' TO 'Superseded';
ALTER TYPE end_reason RENAME VALUE 'timeout' TO 'Timeout';
ALTER TYPE game_result RENAME VALUE 'win' TO 'Win';
ALTER TYPE game_result RENAME VALUE 'lose' TO 'Lose';
ALTER TYPE game_result RENAME VALUE 'remake' TO 'Remake';
ALTER TYPE game_result RENAME VALUE 'unknown' TO 'Unknown';
ALTER TYPE riot_game_source RENAME VALUE 'lcu' TO 'Lcu';
ALTER TYPE riot_game_source RENAME VALUE 'spectator' TO 'Spectator';
ALTER TYPE share_mode RENAME VALUE 'live' TO 'Live';
ALTER TYPE share_mode RENAME VALUE 'public' TO 'Public';
ALTER TYPE share_delete_reason RENAME VALUE 'game_ended' TO 'GameEnded';
ALTER TYPE share_delete_reason RENAME VALUE 'revoked' TO 'Revoked';
ALTER TYPE session_kind RENAME VALUE 'web' TO 'Web';
ALTER TYPE session_kind RENAME VALUE 'desktop' TO 'Desktop';
