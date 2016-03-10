use std::cmp::*;
use std::ops::Add;
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}

fn parse<T : std::str::FromStr>(s : &str) -> T {
    match s.parse::<T>() {
        Ok(t) => t,
        _    => panic!(),
    }
}

fn integral(a: f64, b: f64) -> f64 {
    if a == -1.0 {
        return b.ln();
    } else {
        return b.powf(a + 1.0) / (a + 1.0);
    }
}
fn differential(a: f64, b: f64) -> f64 {
    return b.powf(a - 1.0) * a;
}

fn main() {
    let n: i32 = parse(&getline().trim());
    let b: f64 = parse(&getline().trim());
    let a = getline().trim().split(" ").map(|x| parse(x)).collect::<Vec<f64>>();
    println!("{}", a.iter().map(|x| differential(*x, b)).fold(0.0, Add::add));
    println!("{}", a.iter().map(|x| integral(*x, b)).fold(0.0, Add::add));
}
