use rand::Rng;
use std::{thread, time};

use enigo::*;
use inputbot::KeybdKey::*;
pub mod chat_connector;

struct Movement {
    duration: time::Duration,
    keys: [Key; 2],
}

const LEVEL: i16 = 1;

fn main() {
    let mut chat_connector = chat_connector::ChatConnector::new("cool");

    CapsLockKey.bind(|| {
        while CapsLockKey.is_toggled() {
            seizure();
        }
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

    let max_should_skip: u64 = 3 / LEVEL as u64;
    let max_duration: u64 = 30 * LEVEL as u64;

    let duration = time::Duration::from_millis(rng.gen_range(17..max_duration));

    let mut keys = [Key::Layout('m'), Key::Layout('m')];

    let should_skip = rng.gen_range(0..max_should_skip);
    if should_skip == 0 {
        keys = random_keys(rng.gen_range(0..8));

        return Movement { duration, keys };
    }

    return Movement { duration, keys };
}

fn excecute_movement(movement: Movement) {
    let mut enigo = Enigo::new();

    movement.keys.iter().for_each(|key| enigo.key_down(*key));

    thread::sleep(movement.duration);

    movement.keys.iter().for_each(|key| enigo.key_up(*key));
}

fn random_keys(i: i16) -> [Key; 2] {
    match i {
        0 => return [Key::UpArrow, Key::UpArrow],
        1 => return [Key::UpArrow, Key::RightArrow],
        2 => return [Key::RightArrow, Key::RightArrow],
        3 => return [Key::RightArrow, Key::DownArrow],
        4 => return [Key::DownArrow, Key::DownArrow],
        5 => return [Key::DownArrow, Key::LeftArrow],
        6 => return [Key::LeftArrow, Key::LeftArrow],
        7 => return [Key::LeftArrow, Key::UpArrow],
        _ => return [Key::Layout('m'), Key::Layout('m')],
    }
}
