use std::fs;

fn parse_str(sl: &str, prefix: &str) -> usize {
    sl.strip_prefix(prefix)
        .unwrap()
        .strip_suffix("kB")
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap()
}

#[derive(Default, Debug)]
pub struct RamInfo {
    pub mem_used: usize,
    pub mem_free: usize,
    pub mem_total: usize,
    pub percent_free: f32,
    pub percent_used: f32,
}

impl RamInfo {
    fn fetch(&mut self) {
        let rawdata = fs::read_to_string("/proc/meminfo").expect("error reading /proc/meminfo");

        let mut data = rawdata.lines();

        let memtotal = parse_str(data.nth(0).unwrap(), "MemTotal:");
        let memfree = parse_str(data.nth(0).unwrap(), "MemFree:");

        self.mem_free = memfree;
        self.mem_total = memtotal;
        self.mem_used = memtotal - memfree;

        self.percent_used = (self.mem_used as f32 / self.mem_total as f32) * 100.;
        self.percent_free = 100. - self.percent_used;
    }

    pub fn new() -> Self {
        let mut r = RamInfo::default();
        r.fetch();

        r
    }
}
