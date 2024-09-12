enum Pattern {
    Downlink,
    Uplink,
    Guard,
    Uci,
}

struct Harq {
    id: u16,
}

struct Scheduler {
    numerology: u8,
    step_count: u64,
    pattern: Vec<Pattern>,
    buffer: Vec<u16>,
    harq_pool: Vec<Harq>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            numerology: 0,
            step_count: 0,
            pattern: vec![],
            buffer: vec![],
            harq_pool: vec![],
        }
    }

    pub fn tick(&mut self) {
        self.step_count += 1;
    }
}
