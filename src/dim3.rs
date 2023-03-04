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
