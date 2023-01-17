use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "chipeite", about = "A CHIP-8 emulator")]
pub struct Opt {
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    pub rom_path: PathBuf,
    #[structopt(default_value = "1", short = "s", long = "scale")]
    pub scale_factor: usize,
}
