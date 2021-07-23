use std::sync::{Arc, Mutex, Weak};
use std::thread::{JoinHandle, spawn};
use ::{DeviceState, Keycode};
use super::{KeyboardCallbacks, CallbackGuard};
use ::{DeviceQuery, MouseCallbacks};
use ::{MousePosition, MouseButton};
use MouseState;

pub(crate) struct EventLoop {
    keyboard_callbacks: Arc<KeyboardCallbacks>,
    mouse_callbacks: Arc<MouseCallbacks>,
    _keyboard_thread: JoinHandle<()>,
    _mouse_thread: JoinHandle<()>
}

fn keyboard_thread(callbacks: Weak<KeyboardCallbacks>) -> JoinHandle<()> {
    spawn(move || {
        let device_state = DeviceState::new();
        let mut prev_keys = vec![];
        while let Some(callbacks) = callbacks.upgrade() {
            let keys = device_state.get_keys();
            for key_state in &keys {
                if !prev_keys.contains(key_state) {
                    callbacks.run_key_down(key_state);
                }
            }
            for key_state in &prev_keys {
                if !keys.contains(key_state) {
                    callbacks.run_key_up(key_state);
                }
            }
            prev_keys = keys;
        }
    })
}

fn mouse_thread(callbacks: Weak<MouseCallbacks>) -> JoinHandle<()> {
    spawn(move || {
        let device_state = DeviceState::new();
        let mut previous_mouse_state = MouseState::default();
        while let Some(callbacks) = callbacks.upgrade() {
            let mouse_state = device_state.get_mouse();
            for (index, (previous_state, current_state)) in previous_mouse_state.button_pressed.iter().zip(mouse_state.button_pressed.iter()).enumerate() {
                if *previous_state == false && *current_state == true {
                    callbacks.run_mouse_down(&index);
                } else if *previous_state == true && *current_state == false {
                    callbacks.run_mouse_up(&index);
                }
            }
            if mouse_state.coords != previous_mouse_state.coords {
                callbacks.run_mouse_move(&mouse_state.coords);
            }
            previous_mouse_state = mouse_state;
        }
    })
}

impl Default for EventLoop {
    fn default() -> Self {
        let keyboard_callbacks = Arc::new(KeyboardCallbacks::default());
        let mouse_callbacks = Arc::new(MouseCallbacks::default());
        let _keyboard_thread = keyboard_thread(Arc::downgrade(&keyboard_callbacks));
        let _mouse_thread = mouse_thread(Arc::downgrade(&mouse_callbacks));
        Self { _keyboard_thread, _mouse_thread, keyboard_callbacks, mouse_callbacks }
    }
}

impl EventLoop {
    pub fn on_key_down<Callback: Fn(&Keycode) + Send + Sync + 'static>(&mut self, callback: Callback) -> CallbackGuard<Callback> {
        let _callback = Arc::new(callback);
        self.keyboard_callbacks.push_key_down(_callback.clone());
        CallbackGuard { _callback }
    }

    pub fn on_key_up<Callback: Fn(&Keycode) + Send + Sync + 'static>(&mut self, callback: Callback) -> CallbackGuard<Callback> {
        let _callback = Arc::new(callback);
        self.keyboard_callbacks.push_key_up(_callback.clone());
        CallbackGuard { _callback }
    }

    pub fn on_mouse_move<Callback: Fn(&MousePosition) + Send + Sync + 'static>(&mut self, callback: Callback) -> CallbackGuard<Callback> {
        let _callback = Arc::new(callback);
        self.mouse_callbacks.push_mouse_move(_callback.clone());
        CallbackGuard { _callback }
    }

    pub fn on_mouse_up<Callback: Fn(&MouseButton) + Send + Sync + 'static>(&mut self, callback: Callback) -> CallbackGuard<Callback> {
        let _callback = Arc::new(callback);
        self.mouse_callbacks.push_mouse_up(_callback.clone());
        CallbackGuard { _callback }
    }

    pub fn on_mouse_down<Callback: Fn(&MouseButton) + Send + Sync + 'static>(&mut self, callback: Callback) -> CallbackGuard<Callback> {
        let _callback = Arc::new(callback);
        self.mouse_callbacks.push_mouse_down(_callback.clone());
        CallbackGuard { _callback }
    }
}

lazy_static! {
    pub(crate) static ref EVENT_LOOP: Arc<Mutex<EventLoop>> = Default::default();
}
