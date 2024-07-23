pub mod cpu;
pub mod ram;

use crate::{cpu::CpuInfo, ram::RamInfo};

pub fn get_ram_info() -> RamInfo {
    let ram_info = RamInfo::new();

    ram_info
}

pub fn get_cpu_info() -> CpuInfo {
    let cpu_info = CpuInfo::new();

    cpu_info
}
