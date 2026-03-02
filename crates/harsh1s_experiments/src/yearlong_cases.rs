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
    YearlongCase { date: "2025-08-27", seed: 552940 },
    YearlongCase { date: "2025-08-30", seed: 398910 },
    YearlongCase { date: "2025-09-07", seed: 482543 },
    YearlongCase { date: "2025-09-10", seed: 827080 },
    YearlongCase { date: "2025-09-20", seed: 67038 },
    YearlongCase { date: "2025-09-24", seed: 440951 },
    YearlongCase { date: "2025-10-02", seed: 365904 },
    YearlongCase { date: "2025-10-04", seed: 155372 },
    YearlongCase { date: "2025-10-08", seed: 885709 },
    YearlongCase { date: "2025-10-10", seed: 862894 },
    YearlongCase { date: "2025-10-15", seed: 806544 },
    YearlongCase { date: "2025-10-19", seed: 224703 },
    YearlongCase { date: "2025-10-29", seed: 113618 },
    YearlongCase { date: "2025-11-01", seed: 774573 },
    YearlongCase { date: "2025-11-28", seed: 693562 },
    YearlongCase { date: "2025-11-30", seed: 437254 },
    YearlongCase { date: "2025-12-05", seed: 876019 },
    YearlongCase { date: "2025-12-19", seed: 719088 },
    YearlongCase { date: "2025-12-20", seed: 287820 },
    YearlongCase { date: "2025-12-26", seed: 382391 },
    YearlongCase { date: "2026-01-02", seed: 938510 },
    YearlongCase { date: "2026-01-04", seed: 42205 },
    YearlongCase { date: "2026-01-30", seed: 462760 },
    YearlongCase { date: "2026-01-31", seed: 232454 },
    YearlongCase { date: "2026-02-21", seed: 963674 },
    YearlongCase { date: "2026-03-01", seed: 496147 },
    YearlongCase { date: "2026-03-02", seed: 455133 },
];

pub const fn case_count() -> usize {
    CASES.len()
}
