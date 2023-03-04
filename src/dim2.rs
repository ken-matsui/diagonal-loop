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
