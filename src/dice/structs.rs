#[derive(Debug, Clone)]
pub struct DiceResult {
    pub successes: isize,
    pub failures: usize,
    pub results: Vec<u8>,
}
