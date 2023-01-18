use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "chipeite", about = "A CHIP-8 emulator")]
pub struct Opt {
    #[structopt(default_value = "1", short = "s", long = "scale")]
    pub scale_factor: usize,
    #[structopt(parse(from_os_str))]
    pub rom_path: PathBuf,
}
