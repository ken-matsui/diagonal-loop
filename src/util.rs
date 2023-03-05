use colored::{ColoredString, Colorize};

pub(crate) fn loop_name(dim: &str) -> String {
    format!("{}_dimensional_diagonal_loop", dim)
}

fn title(s: &str) -> ColoredString {
    format!("=== {} ===", s).magenta().bold()
}
pub(crate) fn println_header(s: &str) {
    println!("{}\n", title(s));
}
pub(crate) fn println_footer(s: &str) {
    println!("{}", title(s));
}
