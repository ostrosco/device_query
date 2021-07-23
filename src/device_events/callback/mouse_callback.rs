//! Mouse callback.

use std::sync::{Mutex, Arc, Weak};
use ::MouseButton;
use MousePosition;
use crate::device_events::utils::DrainFilter;

/// Mouse move callback.
pub type MouseMoveCallback = dyn Fn(&MousePosition) + Sync + Send + 'static;

/// Mouse button callback.
pub type MouseButtonCallback = dyn Fn(&MouseButton) + Sync + Send + 'static;

/// Mouse callbacks.
#[derive(Default)]
pub(crate) struct MouseCallbacks {
    pub mouse_move: Mutex<Vec<Weak<MouseMoveCallback>>>,
    pub mouse_up: Mutex<Vec<Weak<MouseButtonCallback>>>,
    pub mouse_down: Mutex<Vec<Weak<MouseButtonCallback>>>,
}

impl MouseCallbacks {
    pub fn push_mouse_move(&self, callback: Arc<MouseMoveCallback>) {
        if let Ok(mut callbacks) = self.mouse_move.lock() {
            let callback = Arc::downgrade(&callback);
            callbacks.push(callback)
        }
    }

    pub fn push_mouse_down(&self, callback: Arc<MouseButtonCallback>) {
        if let Ok(mut callbacks) = self.mouse_down.lock() {
            let callback = Arc::downgrade(&callback);
            callbacks.push(callback)
        }
    }

    pub fn push_mouse_up(&self, callback: Arc<MouseButtonCallback>) {
        if let Ok(mut callbacks) = self.mouse_up.lock() {
            let callback = Arc::downgrade(&callback);
            callbacks.push(callback)
        }
    }

    #[allow(unstable_name_collisions)]
    pub fn run_mouse_move(&self, position: &MousePosition) {
        if let Ok(mut callbacks) = self.mouse_move.lock() {
            callbacks.drain_filter(|callback| callback.upgrade().is_none());
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(position);
                }
            }
        }
    }

    #[allow(unstable_name_collisions)]
    pub fn run_mouse_down(&self, button: &MouseButton) {
        if let Ok(mut callbacks) = self.mouse_down.lock() {
            callbacks.drain_filter(|callback| callback.upgrade().is_none());
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(button);
                }
            }
        }
    }

    #[allow(unstable_name_collisions)]
    pub fn run_mouse_up(&self, button: &MouseButton) {
        if let Ok(mut callbacks) = self.mouse_up.lock() {
            callbacks.drain_filter(|callback| callback.upgrade().is_none());
            for callback in callbacks.iter() {
                if let Some(callback) = callback.upgrade() {
                    callback(button);
                }
            }
        }
    }
}