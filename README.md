[![Rust Docs](https://img.shields.io/badge/docs-rin%20documentation-blue)](https://docs.rs/rin-sys/0.1.0/rin_sys/)

# rin
A small rust library to fetch system information on linux

# Installation

Add ``` rin-sys = "0.1.4" ``` to your Cargo.toml

# Usage

```rust

let ram_info = rin_sys::get_ram_info();
let cpu_info = rin_sys::get_cpu_info();

```

RAM info struct -

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
 
 CPU info struct - 
 ```rust
 #[derive(Debug, Default)]
pub struct CpuInfo {
    pub cache_size: String,
    pub cores: usize,
    pub cpu_speed: Vec<(usize, f64)>,
    pub model_name: String,
    pub vendor_id: String,
    pub is_fpu: bool,
    pub cpuid_level: f32,
}
 ```
