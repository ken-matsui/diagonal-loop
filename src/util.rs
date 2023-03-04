use colored::{ColoredString, Colorize};

fn title(dim: &str) -> ColoredString {
    format!("=== {}_dimensional_diagonal_loop ===", dim)
        .magenta()
        .bold()
}

pub(crate) fn println_header(dim: &str) {
    println!("{}\n", title(dim));
}
pub(crate) fn println_footer(dim: &str) {
    println!("\n{}", title(dim));
}
