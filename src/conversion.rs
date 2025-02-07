use primitive_types::U256;

pub fn sanitize(k: U256) -> U256 {
    let mut k = k.to_little_endian();
    k[31] &= (1 << 7) - 1;
    U256::from_little_endian(&k)
}
pub fn clamp(k: U256) -> U256 {
    let mut k = k.to_little_endian();
    k[0] &= 248;
    k[31] &= 127;
    k[31] |= 64;
    U256::from_little_endian(&k)
}
