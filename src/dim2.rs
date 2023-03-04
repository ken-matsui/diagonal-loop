use colored::Colorize;

#[allow(non_snake_case)]
pub(crate) fn diag_loop(X: usize, Y: usize) {
    #[allow(clippy::needless_range_loop)]
    for x in 0..X {
        for y in 0..Y {
            print!("|{}", format!("({x},{y})").cyan());
        }
        println!("|");
    }

    println!();

    println!("diagonals: {}", count_diagonals(X, Y));
    println!("max diagonal len (middle): {}", max_diagonal_len(X, Y));

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

fn count_diagonals(x: usize, y: usize) -> usize {
    (x + y) - 1
}
fn max_diagonal_len(x: usize, y: usize) -> usize {
    std::cmp::min(x, y)
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
}
