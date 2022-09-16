use std::{thread, time};
use rand::Rng;

use enigo::*;
use inputbot::{KeybdKey::*};


struct Movement {
    duration: time::Duration,
    key: Key
}

const LEVEL: i16 = 1;

fn main() {
    EnterKey.bind(|| while EnterKey.is_toggled() {
        seizure();
    });

    inputbot::handle_input_events();
}

fn seizure() {
    let current_movement = new_movement();

    excecute_movement(current_movement);
}

/** Generates a new move. */
fn new_movement() -> Movement {
    let mut rng = rand::thread_rng();

    let max_should_skip: u64 = 3/LEVEL as u64;
    let max_duration: u64 = 30*LEVEL as u64;

    let duration = time::Duration::from_millis(rng.gen_range(17..max_duration));

    let should_skip = rng.gen_range(0..max_should_skip);
    if should_skip == 0 {
        let key = random_key(rng.gen_range(0..4));

        return Movement {
            duration,
            key
        }
    }

    return Movement {
        duration,
        key: Key::Layout('m')
    }
}

fn excecute_movement(movement: Movement) {
    let mut enigo = Enigo::new();

    enigo.key_down(movement.key);

    thread::sleep(movement.duration);

    enigo.key_up(movement.key);
}

fn random_key(i: i16) -> Key {
    match i {
        0 => return Key::UpArrow,
        1 => return Key::RightArrow,
        2 => return Key::DownArrow,
        3 => return Key::LeftArrow,
        _ => return Key::Escape
    }
}