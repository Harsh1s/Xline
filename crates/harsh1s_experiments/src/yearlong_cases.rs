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
    YearlongCase { date: "2025-06-15", seed: 757391 },
    YearlongCase { date: "2025-06-28", seed: 990758 },
    YearlongCase { date: "2025-07-01", seed: 362014 },
    YearlongCase { date: "2025-07-13", seed: 12228 },
    YearlongCase { date: "2025-07-25", seed: 67228 },
    YearlongCase { date: "2025-08-11", seed: 476522 },
    YearlongCase { date: "2025-08-22", seed: 188786 },
];

pub const fn case_count() -> usize {
    CASES.len()
}
