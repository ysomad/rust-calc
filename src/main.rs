mod error;
mod expr;
mod token;

use expr::Expression;
use std::io;

fn main() {
    let mut raw_expr = String::new();

    io::stdin()
        .read_line(&mut raw_expr)
        .expect("Unable to read expression");

    let trimmed = raw_expr.trim();

    let expr = Expression::new(trimmed).unwrap();
    let res = expr.eval().unwrap();

    println!("> {trimmed} = {res}");
}
