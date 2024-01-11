use crate::character::structs::Stat;

pub fn roll_stat(stat: &Stat) -> Vec<u8> {
    vec![0_u8; stat.quantity]
}
