use std::u8;



pub fn kronecker_delta(i: u8, j: u8) -> u8 {
    return (i == j) as u8;
}
