use std::fs;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CpuInfo {
    pub cache_size: String,
    pub cores: usize,
    pub cpu_speed: String,
    pub model_name: String,
    pub vendor_id: String,
    pub is_fpu: bool,
    pub cpuid_level: usize,
}

impl CpuInfo {
    pub fn new() -> Self {
        let mut cpu_info = CpuInfo::default();
        cpu_info.fetch();
        cpu_info
    }

    pub fn fetch(&mut self) {
        let raw_data =
            fs::read_to_string("/proc/cpuinfo").expect("unable to read from cpuinfo file");
        
        let mut map = HashMap::new();

        for (i, line) in raw_data.lines().enumerate() {
            
            if i >= 20 {

                break;

            }

            let kv: Vec<&str> = line.split(":").collect();

            map.insert(kv[0].trim(), kv[1].trim());

        }
        
        let vendor_id = map.get("vendor_id")
            .expect("could not retrieve vendor id")
            .trim()
            .to_string();
        
        let model_name = map.get("model name")
            .expect("could not retrieve model name") 
            .trim()
            .to_string();

        let cpu_cores = map.get("cpu cores")
            .expect("could not retrieve cpu cores")
            .trim()
            .parse::<usize>()
            .unwrap();

        let cpuid_level = map.get("cpuid level")
            .expect("could not retrieve cpuid level")
            .trim()
            .parse::<usize>()
            .unwrap();

        let cache_size = map.get("cache size")
            .expect("could not retrieve cache size")
            .trim()
            .to_string();

        let is_fpu = if map.get("fpu")
            .expect("could not retrieve fpu")
            .trim() == "yes" {true} else {false};

        let cpu_ghz = model_name
            .split("@")
            .nth(1)
            .unwrap()
            .trim()
            .to_string();

        self.vendor_id = vendor_id;
        self.model_name = model_name;
        self.cpuid_level = cpuid_level;
        self.cores = cpu_cores;
        self.cache_size = cache_size;
        self.cpu_speed = cpu_ghz;
        self.is_fpu = is_fpu;
    }
}
