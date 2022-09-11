use std::u8;

use bevy::prelude::Vec3;

pub fn kronecker_delta(i: u8, j: u8) -> u8 {
    return (i == j) as u8;
}
