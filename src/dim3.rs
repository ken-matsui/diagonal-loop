use crate::util::println_header;
use crate::Opts;

use colored::Colorize;

// TODO: support block
#[allow(non_snake_case)]
pub(crate) fn diag_loop(opts: Opts, X: usize, Y: usize, Z: usize) {
    if !opts.no_elem {
        show_elements(X, Y, Z, opts);
    }
    if opts.report {
        report_diag(X, Y, Z);
    }
}

#[allow(non_snake_case)]
fn show_elements(X: usize, Y: usize, Z: usize, opts: Opts) {
    simple_loop(X, Y, Z);

    if !opts.bottom_up {
        top_down_diag_loop(X, Y, Z);
    } else {
        bottom_up_diag_loop(X, Y, Z);
    }
    println!();
}

#[allow(non_snake_case)]
fn simple_loop(X: usize, Y: usize, Z: usize) {
    #[allow(clippy::needless_range_loop)]
    for z in 0..Z {
        println!("z = {z}");
        for x in 0..X {
            for y in 0..Y {
                print!("|{}", format!("({x},{y},{z})").cyan());
            }
            println!("|");
        }
        println!();
    }
}

#[allow(non_snake_case)]
fn top_down_diag_loop(X: usize, Y: usize, Z: usize) {
    println!("top-down:");
    for offset in 0..=(X + Y + Z - 3) {
        for x in 0..=offset {
            for y in 0..=(offset - x) {
                let z = (offset - x) - y;
                if x < X && y < Y && z < Z {
                    print!("|{}", format!("({x},{y},{z})").cyan());
                }
            }
        }
        println!("|");
    }
}

#[allow(non_snake_case)]
fn bottom_up_diag_loop(X: usize, Y: usize, Z: usize) {
    println!("bottom-up:");
    for offset in 0..=(X + Y + Z - 3) {
        for z in 0..=offset {
            for y in 0..=(offset - z) {
                let x = (offset - z) - y;
                if x < X && y < Y && z < Z {
                    print!("|{}", format!("({x},{y},{z})").cyan());
                }
            }
        }
        println!("|");
    }
}

#[inline]
fn report_diag(x: usize, y: usize, z: usize) {
    println_header("Report");
    println!("num of diagonals: {}", count_diagonals(x, y, z));
    println!("max diagonal len (middle): {}", max_diagonal_len(x, y, z));
    println!("ave diagonal len: {:.2}", ave_diagonal_len(x, y, z));
    println!();
}

#[inline]
fn count_diagonals(x: usize, y: usize, z: usize) -> usize {
    (x + y + z) - 2
}
#[allow(non_snake_case)]
fn max_diagonal_len(X: usize, Y: usize, Z: usize) -> usize {
    // TODO: There might be a cheaper calculation.
    // 1,1,1 => 1
    // 1,1,2 => 1
    // 1,2,2 => 2
    // 2,2,2 => 3
    // 2,2,3 => 4
    // 2,3,3 => 5
    // 3,3,3 => 7
    // 3,3,4 => 8
    // 3,4,4 => 10
    // 4,4,4 => 12
    // 4,4,5 => 14
    // 4,5,5 => 16
    // 5,5,5 => 19
    // 5,5,6 => 21
    // 5,6,6 => 24
    // 6,6,6 => 27
    // 6,6,7 => 30
    // 6,7,7 => 33
    // 7,7,7 => 37
    // 7,7,8 => 40
    // 7,8,8 => 44
    // 8,8,8 => 48
    // 8,8,9 => 52
    // 8,9,9 => 56
    // 9,9,9 => 61
    // 9,9,10 => 65
    // 9,10,10 => 70
    // 10,10,10 => 75
    let mut max_count = 0;
    for offset in 0..=(X + Y + Z - 3) {
        let mut count = 0;
        for z in 0..=offset {
            for y in 0..=(offset - z) {
                let x = (offset - z) - y;
                if x < X && y < Y && z < Z {
                    count += 1;
                }
            }
        }

        if max_count < count {
            max_count = count;
        }
    }
    max_count
}
#[inline]
fn ave_diagonal_len(x: usize, y: usize, z: usize) -> f32 {
    (x * y * z) as f32 / count_diagonals(x, y, z) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    fn count_real_diagonals(X: usize, Y: usize, Z: usize) -> usize {
        let mut count = 0;
        for offset in 0..=(X + Y + Z - 3) {
            for z in 0..=offset {
                for y in 0..=(offset - z) {
                    let x = (offset - z) - y;
                    if x < X && y < Y && z < Z {}
                }
            }
            count += 1;
        }
        count
    }

    #[test]
    fn test_count_diagonals() {
        assert_eq!(count_diagonals(1, 1, 1), count_real_diagonals(1, 1, 1));
        assert_eq!(count_diagonals(2, 2, 2), count_real_diagonals(2, 2, 2));
        assert_eq!(count_diagonals(1, 2, 3), count_real_diagonals(1, 2, 3));
        assert_eq!(count_diagonals(1, 2, 2), count_real_diagonals(1, 2, 2));
        assert_eq!(
            count_diagonals(100, 200, 300),
            count_real_diagonals(100, 200, 300)
        );
    }

    #[allow(non_snake_case)]
    fn real_ave_diagonal_len(X: usize, Y: usize, Z: usize) -> f32 {
        let mut element_count = 0.;
        let mut diag_count = 0.;
        for offset in 0..=(X + Y + Z - 3) {
            for z in 0..=offset {
                for y in 0..=(offset - z) {
                    let x = (offset - z) - y;
                    if x < X && y < Y && z < Z {
                        element_count += 1.;
                    }
                }
            }
            diag_count += 1.;
        }
        element_count / diag_count
    }

    #[test]
    fn test_ave_diagonal_len() {
        assert_eq!(ave_diagonal_len(1, 1, 1), real_ave_diagonal_len(1, 1, 1));
        assert_eq!(ave_diagonal_len(2, 2, 2), real_ave_diagonal_len(2, 2, 2));
        assert_eq!(ave_diagonal_len(1, 2, 3), real_ave_diagonal_len(1, 2, 3));
        assert_eq!(ave_diagonal_len(1, 3, 5), real_ave_diagonal_len(1, 3, 5));
        assert_eq!(
            ave_diagonal_len(2, 100, 500),
            real_ave_diagonal_len(2, 100, 500)
        );
        assert_eq!(
            ave_diagonal_len(100, 200, 300),
            real_ave_diagonal_len(100, 200, 300)
        );
    }
}
