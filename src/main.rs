#![allow(dead_code)]

mod cpu;
mod memory;
mod structopt;
mod video;

use crate::structopt::Opt;
use ::structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opts = Opt::from_args();
    let mut _cpu = cpu::Cpu::new();
    let memory = memory::Memory::new();
    let mut vid = crate::video::Video::new(opts.scale_factor);
    loop {
        let _ = vid.draw(&memory.framebuffer);
        dbg!(vid.wait_for_key());
    }
}
