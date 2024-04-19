#[derive(Debug, Clone)]
pub struct RollResult {
    pub successes: isize,
    pub failures: usize,
    pub results: Vec<u8>,
}

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
#[must_use] pub fn roll_stat(stat: &crate::character::Stat, advantage: usize, disadvantage: usize) -> RollResult {
    let mut quantity = stat.quantity;
    let quality = stat.quality as u8;

    let mut successes = 0;
    let mut failures = 0;

    if advantage > 0 {
        quantity += advantage - 1;
    }

    if disadvantage > 0 {
        // No dice to roll!
        if disadvantage - 1 > quantity {
            return RollResult {
                successes: 0,
                failures: 0,
                results: Vec::new(),
                };
        }
        quantity -= disadvantage - 1;
    }

    let mut results: Vec<u8> = Vec::with_capacity(quantity);

    while quantity > 0 {
        let result: u8 = (rand::random::<u8>() % 6) + 1;
        if advantage > 0 && result == 6 {
            quantity += 1;
        }
        if disadvantage > 0 && result == 1 {
            successes -= 1;
        }
        successes += isize::from(result >= quality);
        failures += usize::from(result < quality);
        results.push(result);
        quantity -= 1;
    }

    RollResult {
        successes,
        failures,
        results,
    }
}
