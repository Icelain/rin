use std::fs;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct CpuInfo {
    pub cache_size: String,
    pub cores: usize,
    // threadwise cpu_mhz for most precision
    pub cpu_speed: Vec<(usize, f64)>,
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
        
        // hashmaps are used so that the code works even after changes in the cpuinfo file
        // vectors would not have been able to keep track of the index of keys properly
        let mut map = HashMap::new();
        
        // cpu_mhz_tracker keeps track of the cpu_mhz at each iteration of the loop as well as the
        // number of iterations
        let mut cpu_mhz_tracker: Vec<(usize, f64)> = vec![];
        
        // l keeps track of no of threads
        let mut l = 0;

        for (_, line) in raw_data.lines().enumerate() {
            
            if !line.contains(":") {
                
                continue;
    
            }

            let kv: Vec<&str> = line.split(":").collect();
            
            let key = kv[0].trim();
            let value = kv[1].trim();
            
            if key == "cpu MHz" { 

                l += 1;
                cpu_mhz_tracker.push((l,value.parse::<f64>().unwrap())); 

                continue
    
            }

            map.insert(key, value);

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

        self.vendor_id = vendor_id;
        self.model_name = model_name;
        self.cpuid_level = cpuid_level;
        self.cores = cpu_cores;
        self.cache_size = cache_size;
        self.cpu_speed = cpu_mhz_tracker;
        self.is_fpu = is_fpu;
    }
}
