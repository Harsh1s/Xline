#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YearlongCase {
    pub date: &'static str,
    pub seed: u32,
}

pub const CASES: &[YearlongCase] = &[
    YearlongCase { date: "2025-06-01", seed: 784552 },
    YearlongCase { date: "2025-06-02", seed: 176025 },
    YearlongCase { date: "2025-06-07", seed: 566471 },
    YearlongCase { date: "2025-06-13", seed: 488919 },
];

pub const fn case_count() -> usize {
    CASES.len()
}
