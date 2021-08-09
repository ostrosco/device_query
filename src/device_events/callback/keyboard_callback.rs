use crate::device_events::utils;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, Weak};
use Keycode;

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

    pub fn run_key_up(&self, key: &Keycode) {
        if let Ok(mut callbacks) = self.key_up.lock() {
            utils::DrainFilter::drain_filter(
                callbacks.deref_mut(),
                |callback| callback.upgrade().is_none(),
            );
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(key);
                }
            }
        }
    }

    pub fn run_key_down(&self, key: &Keycode) {
        if let Ok(mut callbacks) = self.key_down.lock() {
            utils::DrainFilter::drain_filter(
                callbacks.deref_mut(),
                |callback| callback.upgrade().is_none(),
            );
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(key);
                }
            }
        }
    }
}
