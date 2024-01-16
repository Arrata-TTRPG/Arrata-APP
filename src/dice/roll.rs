use crate::character::structs::Stat;
use crate::dice::structs::*;

/// Rolls a given stat with advantage and disadvantage.
///
/// # Inputs
///
/// `stat: Stat` - The stat to roll.
///
/// `advantage: usize` - The level of advantage on the roll.
///
/// `disadvantage: usize` - The level of disadvantage on the roll.
///
/// # Outputs
///
/// `DiceResult` - The result of the roll.
pub fn roll_stat(stat: Stat, advantage: usize, disadvantage: usize) -> DiceResult {
    let mut quantity: isize = stat.quantity as isize;
    let quality = stat.quality as u8;

    let mut successes = 0;
    let mut failures = 0;

    if advantage > 0 {
        quantity += advantage as isize - 1;
    }

    if disadvantage > 0 {
        quantity -= disadvantage as isize - 1;
    }

    let mut results: Vec<u8> = Vec::with_capacity(quantity.max(0) as usize);

    while quantity > 0 {
        let result: u8 = (rand::random::<u8>() % 6) + 1;
        if advantage > 0 && result == 6 {
            quantity += 1;
        }
        if disadvantage > 0 && result == 1 {
            successes -= 1;
        }
        successes += (result >= quality) as isize;
        failures += (result < quality) as usize;
        results.push(result);
        quantity -= 1;
    }

    DiceResult {
        successes,
        failures,
        results,
    }
}
