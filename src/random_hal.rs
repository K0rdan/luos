extern crate rand;

use self::rand::Rng;

pub fn digital_read(_pin: u8) -> u8 {
    rand::thread_rng().gen::<bool>() as u8
}

pub fn digital_write(_pin: u8, _value: u8) {}
