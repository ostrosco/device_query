use Keycode;
use std::sync::{Mutex, Arc, Weak};
use crate::device_events::utils::DrainFilter;

/// Keyboard callback.
pub type KeyboardCallback = dyn Fn(&Keycode) + Sync + Send + 'static;

/// Keyboard callbacks.
#[derive(Default)]
pub(crate) struct KeyboardCallbacks {
    key_down: Mutex<Vec<Weak<KeyboardCallback>>>,
    key_up: Mutex<Vec<Weak<KeyboardCallback>>>,
}

impl KeyboardCallbacks {
    pub fn push_key_up(&self, callback: Arc<KeyboardCallback>) {
        if let Ok(mut key_down) = self.key_up.lock() {
            let callback = Arc::downgrade(&callback);
            key_down.push(callback)
        }
    }

    pub fn push_key_down(&self, callback: Arc<KeyboardCallback>) {
        if let Ok(mut key_down) = self.key_down.lock() {
            let callback = Arc::downgrade(&callback);
            key_down.push(callback)
        }
    }

    #[allow(unstable_name_collisions)]
    pub fn run_key_up(&self, key: &Keycode) {
        if let Ok(mut callbacks) = self.key_up.lock() {
            callbacks.drain_filter(|callback| callback.upgrade().is_none());
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(key);
                }
            }
        }
    }

    #[allow(unstable_name_collisions)]
    pub fn run_key_down(&self, key: &Keycode) {
        if let Ok(mut callbacks) = self.key_down.lock() {
            callbacks.drain_filter(|callback| callback.upgrade().is_none());
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(key);
                }
            }
        }
    }
}
