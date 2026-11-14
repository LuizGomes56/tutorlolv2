//! A list whose entries keep their identity: the web app's `utils/tray.rs`.
use std::sync::atomic::{AtomicU64, Ordering};

/// Values with stable identities, so one of two equal entries can be removed.
#[derive(Clone, Debug)]
pub struct Tray<T: Copy> {
    ids: Vec<u64>,
    values: Vec<T>,
}

impl<T: Copy> Default for Tray<T> {
    fn default() -> Self {
        Self {
            ids: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl<T: Copy> Tray<T> {
    pub fn values(&self) -> &[T] {
        &self.values
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn push(&mut self, value: T) {
        // The web uses random ids. Unique ones are enough.
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);

        self.ids.push(NEXT_ID.fetch_add(1, Ordering::Relaxed));
        self.values.push(value);
    }

    /// Each value with its id, in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (u64, T)> + '_ {
        self.ids.iter().copied().zip(self.values.iter().copied())
    }

    pub fn remove_by_id(&mut self, id: u64) {
        if let Some(index) = self.ids.iter().position(|&other| other == id) {
            self.ids.remove(index);
            self.values.remove(index);
        }
    }

    pub fn clear(&mut self) {
        self.ids.clear();
        self.values.clear();
    }

    /// Keeps the values for which `keep` is true, with their ids.
    pub fn retain(&mut self, mut keep: impl FnMut(T) -> bool) {
        let mut index = 0;

        while index < self.values.len() {
            if keep(self.values[index]) {
                index += 1;
            } else {
                self.ids.remove(index);
                self.values.remove(index);
            }
        }
    }
}

impl<T: Copy> FromIterator<T> for Tray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tray = Self::default();

        for value in iter {
            tray.push(value);
        }

        tray
    }
}
