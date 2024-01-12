use crate::character::structs::Stat;
use crate::dice::structs::*;

pub fn roll_stat(stat: &Stat) -> DiceResult {
    let quality = stat.quality.clone() as u8;
    let mut successes = 0;

    let results = (0..stat.quantity)
        .map(|_| {
            let result: u8 = rand::random::<u8>();
            successes += (result >= quality) as usize;
            result
        })
        .collect::<Vec<u8>>();

    DiceResult {
        successes,
        failures: stat.quantity - successes,
        results,
    }
}
