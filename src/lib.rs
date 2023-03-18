use std::sync::{Arc, Mutex};

use rdev::{simulate, EventType, Key};

pub mod maps;
pub mod midi;

pub type Data<T> = Arc<Mutex<T>>;

pub fn data_new<T>(val: T) -> Data<T> {
    Arc::new(Mutex::new(val))
}

#[inline]
fn press(event: &EventType) {
    simulate(event).unwrap();
}

#[inline]
pub fn send(key: Key) {
    press(&EventType::KeyPress(key));
    press(&EventType::KeyRelease(key));
}
