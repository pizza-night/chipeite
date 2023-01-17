mod framebuffer;
mod cpu;

fn main() -> anyhow::Result<()> {
    let cpu = cpu::Cpu::new(std::env::args().nth(1).unwrap())?;
    Ok(())
}
