mod dim2;
mod dim3;
mod util;

use util::{loop_name, println_footer, println_header};

use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// the length of x-axis
    x: usize,
    /// the length of y-axis
    y: usize,
    /// the length of z-axis (for a 3-dim array)
    z: Option<usize>,

    /// Show general information about diagonals
    #[clap(short, long)]
    report: bool,

    /// Do not show elements
    #[clap(long)]
    no_elem: bool,

    /// Loop diagonals bottom-up (default: top-down)
    #[clap(long)]
    bottom_up: bool,

    /// Split by blocks (BLOCK*BLOCK)
    #[clap(long)]
    block: Option<usize>,
}

#[derive(Copy, Clone)]
pub(crate) struct Opts {
    report: bool,
    no_elem: bool,
    bottom_up: bool,
    block: Option<usize>,
}

impl From<&Cli> for Opts {
    fn from(value: &Cli) -> Self {
        Self {
            report: value.report,
            no_elem: value.no_elem,
            bottom_up: value.bottom_up,
            block: value.block,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(block) = cli.block {
        assert!(block <= cli.x, "The block size must be <= X");
        assert!(block <= cli.y, "The block size must be <= Y");
        if let Some(z) = cli.z {
            assert!(block <= z, "The block size must be <= Z");
        }
    }

    if let Some(z) = cli.z {
        println_header(&loop_name("three"));
        dim3::diag_loop(Opts::from(&cli), cli.x, cli.y, z);
        println_footer(&loop_name("three"));
    } else {
        println_header(&loop_name("two"));
        dim2::diag_loop(Opts::from(&cli), cli.x, cli.y);
        println_footer(&loop_name("two"));
    }
}
