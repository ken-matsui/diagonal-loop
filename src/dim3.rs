use colored::Colorize;

#[allow(non_snake_case)]
pub(crate) fn diag_loop(X: usize, Y: usize, Z: usize) {
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

    println!("{} diagonals:", count_diagonals(X, Y, Z));
    println!("ave diagonal len: {:.2}", ave_diagonal_len(X, Y, Z));

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
fn count_diagonals(x: usize, y: usize, z: usize) -> usize {
    (x + y + z) - 2
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
