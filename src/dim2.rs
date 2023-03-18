use crate::util::println_header;
use crate::Opts;

use colored::Colorize;

#[allow(non_snake_case)]
pub(crate) fn diag_loop(opts: Opts, X: usize, Y: usize) {
    if !opts.no_elem {
        show_elements(X, Y, opts.bottom_up);
    }
    if opts.report {
        report_diag(X, Y);
    }
}

#[allow(non_snake_case)]
fn show_elements(X: usize, Y: usize, bottom_up: bool) {
    simple_loop(X, Y);
    println!();

    if bottom_up {
        top_down_diag(X, Y);
    } else {
        bottom_up_diag(X, Y);
    }
    println!();
}

#[allow(non_snake_case)]
fn simple_loop(X: usize, Y: usize) {
    #[allow(clippy::needless_range_loop)]
    for x in 0..X {
        for y in 0..Y {
            print!("|{}", format!("({x},{y})").cyan());
        }
        println!("|");
    }
}

#[allow(non_snake_case)]
fn top_down_diag(X: usize, Y: usize) {
    println!("top-down:");
    for offset in 0..=(X + Y - 2) {
        for x in 0..=offset {
            let y = offset - x;
            if x < X && y < Y {
                print!("|{}", format!("({x},{y})").cyan());
            }
        }
        println!("|");
    }
}

#[allow(non_snake_case)]
fn bottom_up_diag(X: usize, Y: usize) {
    println!("bottom-up:");
    for offset in 0..=(X + Y - 2) {
        for y in 0..=offset {
            let x = offset - y;
            if x < X && y < Y {
                print!("|{}", format!("({x},{y})").cyan());
            }
        }
        println!("|");
    }
}

#[inline]
fn report_diag(x: usize, y: usize) {
    println_header("Report");
    println!("num of diagonals: {}", count_diagonals(x, y));
    println!("max diagonal len (middle): {}", max_diagonal_len(x, y));
    println!("ave diagonal len: {:.2}", ave_diagonal_len(x, y));
    println!();
}

#[inline]
fn count_diagonals(x: usize, y: usize) -> usize {
    (x + y) - 1
}
#[inline]
fn max_diagonal_len(x: usize, y: usize) -> usize {
    std::cmp::min(x, y)
}
#[inline]
fn ave_diagonal_len(x: usize, y: usize) -> f32 {
    (x * y) as f32 / count_diagonals(x, y) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    fn real_count_diagonals(X: usize, Y: usize) -> usize {
        let mut count = 0;
        for offset in 0..=(X + Y - 2) {
            for y in 0..=offset {
                let x = offset - y;
                if x < X && y < Y {}
            }
            count += 1;
        }
        count
    }

    #[test]
    fn test_count_diagonals() {
        assert_eq!(count_diagonals(1, 1), real_count_diagonals(1, 1));
        assert_eq!(count_diagonals(2, 2), real_count_diagonals(2, 2));
        assert_eq!(count_diagonals(1, 2), real_count_diagonals(1, 2));
        assert_eq!(count_diagonals(1, 3), real_count_diagonals(1, 3));
        assert_eq!(count_diagonals(2, 100), real_count_diagonals(2, 100));
        assert_eq!(count_diagonals(100, 200), real_count_diagonals(100, 200));
    }

    #[allow(non_snake_case)]
    fn real_max_diagonal_len(X: usize, Y: usize) -> usize {
        let mut max_count = 0;
        for offset in 0..=(X + Y - 2) {
            let mut count = 0;
            for y in 0..=offset {
                let x = offset - y;
                if x < X && y < Y {
                    count += 1;
                }
            }

            if max_count < count {
                max_count = count;
            }
        }
        max_count
    }

    #[test]
    fn test_max_diagonal_len() {
        assert_eq!(max_diagonal_len(1, 1), real_max_diagonal_len(1, 1));
        assert_eq!(max_diagonal_len(2, 2), real_max_diagonal_len(2, 2));
        assert_eq!(max_diagonal_len(1, 2), real_max_diagonal_len(1, 2));
        assert_eq!(max_diagonal_len(1, 3), real_max_diagonal_len(1, 3));
        assert_eq!(max_diagonal_len(2, 100), real_max_diagonal_len(2, 100));
        assert_eq!(max_diagonal_len(100, 200), real_max_diagonal_len(100, 200));
    }

    #[allow(non_snake_case)]
    fn real_ave_diagonal_len(X: usize, Y: usize) -> f32 {
        let mut element_count = 0.;
        let mut diag_count = 0.;
        for offset in 0..=(X + Y - 2) {
            for y in 0..=offset {
                let x = offset - y;
                if x < X && y < Y {
                    element_count += 1.;
                }
            }
            diag_count += 1.;
        }
        element_count / diag_count
    }

    #[test]
    fn test_ave_diagonal_len() {
        assert_eq!(ave_diagonal_len(1, 1), real_ave_diagonal_len(1, 1));
        assert_eq!(ave_diagonal_len(2, 2), real_ave_diagonal_len(2, 2));
        assert_eq!(ave_diagonal_len(1, 2), real_ave_diagonal_len(1, 2));
        assert_eq!(ave_diagonal_len(1, 3), real_ave_diagonal_len(1, 3));
        assert_eq!(ave_diagonal_len(2, 100), real_ave_diagonal_len(2, 100));
        assert_eq!(ave_diagonal_len(100, 200), real_ave_diagonal_len(100, 200));
    }
}
