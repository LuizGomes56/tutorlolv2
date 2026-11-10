//! Latest-only live fan-out. Database capabilities remain authoritative.
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Instant,
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore, watch};
use uuid::Uuid;

#[derive(Clone)]
pub struct LiveFrame {
    pub seq: u64,
    pub captured_at: i64,
    pub game_time: f32,
    pub bytes: Vec<u8>,
    received: Instant,
}

impl LiveFrame {
    pub fn age_ms(&self) -> u64 {
        self.received.elapsed().as_millis() as u64
    }
}

struct Channel {
    sender: watch::Sender<Option<Arc<LiveFrame>>>,
    viewers: Arc<Semaphore>,
    seq: u64,
}

impl Channel {
    fn new(max_viewers: usize) -> Self {
        Self {
            sender: watch::channel(None).0,
            viewers: Arc::new(Semaphore::new(max_viewers)),
            seq: 0,
        }
    }
}

#[derive(Default)]
pub struct LiveHub {
    channels: Mutex<HashMap<Uuid, Channel>>,
}

impl LiveHub {
    pub fn publish(
        &self,
        id: Uuid,
        captured_at: i64,
        game_time: f32,
        bytes: Vec<u8>,
        max_viewers: usize,
    ) {
        let mut channels = self.channels.lock().unwrap();
        let channel = channels
            .entry(id)
            .or_insert_with(|| Channel::new(max_viewers));

        channel.seq = channel
            .seq
            .saturating_add(1)
            .max(chrono::Utc::now().timestamp_millis().max(1) as u64);

        let frame = Arc::new(LiveFrame {
            seq: channel.seq,
            captured_at,
            game_time,
            bytes,
            received: Instant::now(),
        });

        channel.sender.send_replace(Some(frame));
    }

    pub fn latest(&self, id: Uuid) -> Option<Arc<LiveFrame>> {
        self.channels
            .lock()
            .unwrap()
            .get(&id)
            .and_then(|c| c.sender.borrow().clone())
    }

    pub fn subscribe(
        &self,
        id: Uuid,
        max_viewers: usize,
    ) -> Option<(
        watch::Receiver<Option<Arc<LiveFrame>>>,
        OwnedSemaphorePermit,
    )> {
        let mut channels = self.channels.lock().unwrap();
        let channel = channels
            .entry(id)
            .or_insert_with(|| Channel::new(max_viewers));

        let permit = channel.viewers.clone().try_acquire_owned().ok()?;

        Some((channel.sender.subscribe(), permit))
    }

    /// Called after the database transaction commits: pending viewers recheck
    /// their exact code instead of inheriting access from a new capability.
    pub fn close(&self, id: Uuid) {
        if let Some(channel) = self.channels.lock().unwrap().remove(&id) {
            channel.sender.send_replace(None);
        }
    }
}
