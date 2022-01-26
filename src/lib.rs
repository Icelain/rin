pub mod cpu;
pub mod ram;

use crate::{ram::RamInfo, cpu::CpuInfo};

pub fn get_ram_info() -> RamInfo {

    let mut ram_info = RamInfo::new();
    ram_info.fetch();

    ram_info

}

pub fn get_cpu_info() -> CpuInfo {

    let mut cpu_info = CpuInfo::new();
    cpu_info.fetch();

    cpu_info

}
