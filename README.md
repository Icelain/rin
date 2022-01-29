[![Rust Docs](https://img.shields.io/badge/docs-rin%20documentation-blue)](https://docs.rs/rin-sys/0.1.0/rin_sys/)

# rin
A small rust library to fetch system information on linux

# Installation

Add ``` rin-sys = "0.1.2" ``` to your Cargo.toml

# Base structs

RAM info  -

 ```rust
 #[derive(Default, Debug)]
pub struct RamInfo {
    pub mem_used: usize,
    pub mem_free: usize,
    pub mem_total: usize,
    pub percent_free: f32,
    pub percent_used: f32,
}
 ```
 
 CPU info - 
 ```rust
 #[derive(Debug, Default)]
pub struct CpuInfo {
    pub cache_size: String,
    pub cores: usize,
    pub cpu_speed: String,
    pub model_name: String,
    pub vendor_id: String,
    pub is_fpu: bool,
    pub cpuid_level: f32,
}
 ```
