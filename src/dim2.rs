use crate::util::println_header;
use crate::Opts;

use colored::Colorize;

#[allow(non_snake_case)]
pub(crate) fn diag_loop(opts: Opts, X: usize, Y: usize) {
    if !opts.no_elem {
        show_elements(X, Y, opts);
    }
    if opts.report {
        report_diag(X, Y, opts.block);
    }
}

#[allow(non_snake_case)]
fn show_elements(X: usize, Y: usize, opts: Opts) {
    simple_loop(X, Y);
    println!();

    if !opts.bottom_up {
        if let Some(block) = opts.block {
            top_down_blocked_diag_loop(X, Y, block);
        } else {
            top_down_diag_loop(X, Y);
        }
    } else {
        #[allow(clippy::collapsible-else-if)]
        if let Some(block) = opts.block {
            bottom_up_blocked_diag_loop(X, Y, block);
        } else {
            bottom_up_diag_loop(X, Y);
        }
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
fn top_down_diag_loop(X: usize, Y: usize) {
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
fn top_down_blocked_diag_loop(X: usize, Y: usize, block: usize) {
    println!("top-down:");
    for offset in (0..=(X + Y - 2)).step_by(block) {
        for x in (0..=offset).step_by(block) {
            let y = offset - x;
            if x < X && y < Y {
                let to_x = if x + block < X { x + block } else { X } - 1; // remove -1 to make exclusive
                let to_y = if y + block < Y { y + block } else { Y } - 1; // remove -1 to make exclusive
                print!("|{}", format!("({x},{y})..=({to_x},{to_y})").cyan());
            }
        }
        println!("|");
    }
}

#[allow(non_snake_case)]
fn bottom_up_diag_loop(X: usize, Y: usize) {
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

#[allow(non_snake_case)]
fn bottom_up_blocked_diag_loop(X: usize, Y: usize, block: usize) {
    println!("bottom-up:");
    for offset in (0..=(X + Y - 2)).step_by(block) {
        for y in (0..=offset).step_by(block) {
            let x = offset - y;
            if x < X && y < Y {
                let to_x = if x + block < X { x + block } else { X } - 1; // remove -1 to make exclusive
                let to_y = if y + block < Y { y + block } else { Y } - 1; // remove -1 to make exclusive
                print!("|{}", format!("({x},{y})..=({to_x},{to_y})").cyan());
            }
        }
        println!("|");
    }
}

#[inline]
fn report_diag(x: usize, y: usize, b: Option<usize>) {
    println_header("Report");
    println!("num of diagonals: {}", count_diagonals(x, y, b));
    println!("max diagonal len (middle): {}", max_diagonal_len(x, y, b));
    println!("ave diagonal len: {:.2}", ave_diagonal_len(x, y, b));
    println!();
}

#[inline]
fn count_diagonals(x: usize, y: usize, block: Option<usize>) -> usize {
    let block = block.unwrap_or(1);
    (x / block + y / block) - 1
}
#[inline]
fn max_diagonal_len(x: usize, y: usize, block: Option<usize>) -> usize {
    let block = block.unwrap_or(1);
    std::cmp::min(x / block, y / block)
}
#[inline]
fn ave_diagonal_len(x: usize, y: usize, block: Option<usize>) -> f32 {
    let b = block.unwrap_or(1);
    (x / b * y / b) as f32 / count_diagonals(x, y, block) as f32
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
        assert_eq!(count_diagonals(1, 1, None), real_count_diagonals(1, 1));
        assert_eq!(count_diagonals(2, 2, None), real_count_diagonals(2, 2));
        assert_eq!(count_diagonals(1, 2, None), real_count_diagonals(1, 2));
        assert_eq!(count_diagonals(1, 3, None), real_count_diagonals(1, 3));
        assert_eq!(count_diagonals(2, 100, None), real_count_diagonals(2, 100));
        assert_eq!(
            count_diagonals(100, 200, None),
            real_count_diagonals(100, 200)
        );
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
        assert_eq!(max_diagonal_len(1, 1, None), real_max_diagonal_len(1, 1));
        assert_eq!(max_diagonal_len(2, 2, None), real_max_diagonal_len(2, 2));
        assert_eq!(max_diagonal_len(1, 2, None), real_max_diagonal_len(1, 2));
        assert_eq!(max_diagonal_len(1, 3, None), real_max_diagonal_len(1, 3));
        assert_eq!(
            max_diagonal_len(2, 100, None),
            real_max_diagonal_len(2, 100)
        );
        assert_eq!(
            max_diagonal_len(100, 200, None),
            real_max_diagonal_len(100, 200)
        );
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
        assert_eq!(ave_diagonal_len(1, 1, None), real_ave_diagonal_len(1, 1));
        assert_eq!(ave_diagonal_len(2, 2, None), real_ave_diagonal_len(2, 2));
        assert_eq!(ave_diagonal_len(1, 2, None), real_ave_diagonal_len(1, 2));
        assert_eq!(ave_diagonal_len(1, 3, None), real_ave_diagonal_len(1, 3));
        assert_eq!(
            ave_diagonal_len(2, 100, None),
            real_ave_diagonal_len(2, 100)
        );
        assert_eq!(
            ave_diagonal_len(100, 200, None),
            real_ave_diagonal_len(100, 200)
        );
    }
}
