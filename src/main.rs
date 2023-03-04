mod dim2;
mod dim3;
mod util;

use util::{println_footer, println_header};

use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// the length of x-axis
    x: usize,
    /// the length of y-axis
    y: usize,
    /// the length of z-axis (for a 3-dimensional array)
    z: Option<usize>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(z) = cli.z {
        println_header("three");
        dim3::diag_loop(cli.x, cli.y, z);
        println_footer("three");
    } else {
        println_header("two");
        dim2::diag_loop(cli.x, cli.y);
        println_footer("two");
    }
}
