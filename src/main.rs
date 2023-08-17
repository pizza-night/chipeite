mod cpu;
mod memory;
mod structopt;
mod video;

use std::fs;

use crate::structopt::Opt;
use ::structopt::StructOpt;
use cpu::Cpu;
use memory::Memory;
use video::Video;

fn main() -> anyhow::Result<()> {
    let opts = Opt::from_args();
    let mut cpu = Cpu::new();
    let mut memory = Memory::new(&fs::read(&opts.rom_path)?, Video::new(opts.scale_factor));
    loop {
        cpu.execute(&mut memory);
    }
}
