//! Match identity (spec §8.3): deciding whether two snapshots belong to the
//! same match, without any match id from port 2999.

use crate::{GameMode, Region, Team};
use alloc::{collections::BTreeMap, string::String, vec::Vec};
use bincode::{Decode, Encode};
use sha2::{Digest, Sha256};

/// `gameTime` may go back this much (seconds) inside one match.
const BACKWARDS_TOLERANCE_S: f32 = 2.0;
/// `gameTime` may outrun the wall clock by this much (seconds).
const FASTER_TOLERANCE_S: f32 = 10.0;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode)]
pub struct RosterEntry {
    pub team: Team,
    pub riot_id: String,
    /// Lowercase internal key from `champion_key`, e.g. `"neeko"`, `"monkeyking"`.
    pub champion: String,
    pub skin_id: u16,
    /// `None` in modes without runes (ARAM: Mayhem, Arena).
    pub keystone: Option<u32>,
    pub is_bot: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode)]
pub struct EventKey {
    pub id: u32,
    /// FNV-1a of `EventName`.
    pub name_hash: u32,
    /// `(EventTime as f32).to_bits()`.
    pub time_bits: u32,
}

/// The real Riot game, as the League client reports it.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Encode, Decode)]
pub struct RiotGameRef {
    pub region: Region,
    pub game_id: u64,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
pub struct Probe {
    pub active_riot_id: String,
    pub game_mode: GameMode,
    pub map_number: u8,
    /// Sorted.
    pub roster: Vec<RosterEntry>,
    pub game_time: f32,
    /// In API order.
    pub events: Vec<EventKey>,
    pub riot_game: Option<RiotGameRef>,
}

/// Why two probes can't belong to the same match.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DifferentReason {
    RiotGameDiffers,
    AccountChanged,
    ModeChanged,
    RosterChanged,
    ChampionChanged,
    TimeWentBackwards,
    TimeTooFast,
    EventsDiverged,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MatchDecision {
    Same,
    Different(DifferentReason),
}

/// SHA-256 over the parts of a probe that never change during a match. Used
/// as an index key, never as the sole same/different decision.
pub fn fingerprint(p: &Probe) -> [u8; 32] {
    let mut roster = p.roster.clone();

    roster.sort();

    let bytes = bincode::encode_to_vec(
        (&p.game_mode, p.map_number, &roster),
        tutorlolv2_types::BINCODE_CONFIG,
    )
    .expect("encoding into a Vec cannot fail");

    Sha256::digest(bytes).into()
}

impl Probe {
    /// `next` is the newer probe of the same observer; `prev_seen_ms` and `now_ms`
    /// are that observer's wall clock (spec §8.3, rules 1–7 in order).
    pub fn same_match(&self, prev_seen_ms: i64, next: &Self, now_ms: i64) -> MatchDecision {
        if let (Some(a), Some(b)) = (&self.riot_game, &next.riot_game) {
            return if a == b {
                MatchDecision::Same
            } else {
                MatchDecision::Different(DifferentReason::RiotGameDiffers)
            };
        }

        let result = self
            .identity(next)
            .and_then(|_| self.roster_grew(next))
            .and_then(|_| self.champions(next))
            .and_then(|_| self.times(prev_seen_ms, next, now_ms))
            .and_then(|_| self.shared_events_agree(next))
            .and_then(|_| self.events_kept(next));

        match result {
            Ok(()) => MatchDecision::Same,
            Err(reason) => MatchDecision::Different(reason),
        }
    }

    /// Order-free identity check for data that may arrive late or out of order
    /// (server-side chunk validation): no time rules, and either roster may be the
    /// larger one.
    pub fn consistent(&self, b: &Self) -> Result<(), DifferentReason> {
        if let (Some(x), Some(y)) = (&self.riot_game, &b.riot_game) {
            return if x == y {
                Ok(())
            } else {
                Err(DifferentReason::RiotGameDiffers)
            };
        }

        self.identity(b)?;
        self.roster_grew(b).or_else(|_| b.roster_grew(self))?;
        self.champions(b)?;
        self.shared_events_agree(b)
    }

    fn identity(&self, next: &Self) -> Result<(), DifferentReason> {
        if self.active_riot_id != next.active_riot_id {
            return Err(DifferentReason::AccountChanged);
        }

        if self.game_mode != next.game_mode || self.map_number != next.map_number {
            return Err(DifferentReason::ModeChanged);
        }

        Ok(())
    }

    /// Rule 4: nobody can join a running match; entries that disappear are fine.
    fn roster_grew(&self, next: &Self) -> Result<(), DifferentReason> {
        let mut seats = BTreeMap::<(&Team, &str), i32>::new();

        for e in &self.roster {
            *seats.entry((&e.team, e.riot_id.as_str())).or_default() += 1;
        }

        for e in &next.roster {
            let seat = seats.entry((&e.team, e.riot_id.as_str())).or_default();
            *seat -= 1;

            if *seat < 0 {
                return Err(DifferentReason::RosterChanged);
            }
        }

        Ok(())
    }

    /// Rule 5: a player present in both probes keeps their champion.
    fn champions(&self, b: &Self) -> Result<(), DifferentReason> {
        for e in &b.roster {
            let mut same_seat = self
                .roster
                .iter()
                .filter(|x| x.team == e.team && x.riot_id == e.riot_id)
                .peekable();

            if same_seat.peek().is_some() && !same_seat.any(|x| x.champion == e.champion) {
                return Err(DifferentReason::ChampionChanged);
            }
        }

        Ok(())
    }

    /// Rule 6: game time can't go backwards nor outrun the wall clock.
    fn times(&self, prev_seen_ms: i64, next: &Self, now_ms: i64) -> Result<(), DifferentReason> {
        let dg = next.game_time - self.game_time;
        let dw = (now_ms - prev_seen_ms) as f32 / 1000.0;

        if dg < -BACKWARDS_TOLERANCE_S {
            return Err(DifferentReason::TimeWentBackwards);
        }

        if dg > dw + FASTER_TOLERANCE_S {
            return Err(DifferentReason::TimeTooFast);
        }

        Ok(())
    }

    /// Rule 7, first half: an event id present in both has the same name and time.
    fn shared_events_agree(&self, b: &Self) -> Result<(), DifferentReason> {
        let by_id = self
            .events
            .iter()
            .map(|e| (e.id, e))
            .collect::<BTreeMap<u32, &EventKey>>();

        match b
            .events
            .iter()
            .all(|e| by_id.get(&e.id).is_none_or(|x| *x == e))
        {
            true => Ok(()),
            false => Err(DifferentReason::EventsDiverged),
        }
    }

    /// Rule 7, second half: events are append-only, so `next` keeps every event of `self`.
    fn events_kept(&self, next: &Self) -> Result<(), DifferentReason> {
        let ids = next
            .events
            .iter()
            .map(|e| (e.id, ()))
            .collect::<BTreeMap<u32, ()>>();

        match self.events.iter().all(|e| ids.contains_key(&e.id)) {
            true => Ok(()),
            false => Err(DifferentReason::EventsDiverged),
        }
    }
}
