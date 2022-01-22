use crate::common::SysInfo;
use std::fs;

#[derive(Debug, Default)]
pub struct CpuInfo {

    pub cache_size: usize,
    pub cores: usize,
    pub cpu_mhz: f32,
    pub model_name: String,
    pub vendor_id: String,
    pub is_fpu: bool,
    pub cpuid_level: f32 

}

fn parse_str(data: &str) -> String {

   let s = data.to_string();
    
   s.split(":")
       .collect::<Vec<&str>>()[1]
       .to_string()
       .trim()
       .to_string()

}

impl SysInfo for CpuInfo {

    fn new() -> Self {

        let mut cpu_info = CpuInfo{..Default::default()};
        cpu_info.fetch();

        cpu_info

    }

    fn fetch(&mut self) {

        let raw_data = fs::read("/proc/cpuinfo")
            .expect("unable to read from cpuinfo file");

        let d = String::from_utf8(raw_data).unwrap();

        let data: Vec<&str> = d.split("\n").collect();

        let vendor_id = parse_str(data[1]);
        let model_name = parse_str(data[4]);

        let cpuid_level = parse_str(data[21])
            .parse::<f32>()
            .unwrap();
        let cpu_cores = parse_str(data[12])
            .parse::<usize>()
            .unwrap();
        let cache_size = parse_str(data[10])
            .trim_end_matches("KB")
            .trim()
            .parse::<usize>()
            .unwrap();

        let cpu_mhz = parse_str(data[7])
            .parse::<f32>()
            .unwrap(); 
    
        let is_fpu = if parse_str(data[15]).as_str() == "yes" {true} else {false}; 

        self.vendor_id = vendor_id;
        self.model_name = model_name;
        self.cpuid_level = cpuid_level;
        self.cores = cpu_cores;
        self.cache_size = cache_size;
        self.cpu_mhz = cpu_mhz;
        self.is_fpu = is_fpu;

    }

}
