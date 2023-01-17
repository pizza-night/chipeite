mod cpu;
mod framebuffer;
mod structopt;
mod video;

use crate::framebuffer::FrameBuffer;
use crate::structopt::Opt;
use ::structopt::StructOpt;

fn main() -> anyhow::Result<()> {
    let opts = Opt::from_args();
    let mut _cpu = cpu::Cpu::new(opts.rom_path)?;
    let mut vid = crate::video::Video::new(opts.scale_factor);
    loop {
        let _ = vid.draw(&FrameBuffer::default());
        dbg!(vid.wait_for_key());
    }
    //Ok(())
}
