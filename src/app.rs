use crate::disks::BlockDevice;
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'O', long, action = ArgAction::SetTrue)]
    /// output all columns
    pub output_all: bool,

    #[arg(short, long)]
    /// output columns see (--list-columns)
    pub output: Option<String>
}

#[derive(Debug)]
pub struct App {
    pub disks: Vec<BlockDevice>,
    pub args: Args
}

impl App {
    pub fn new(disks: Vec<BlockDevice>) -> Self {
        let args = Args::parse();

        Self { disks, args }
    }
}
