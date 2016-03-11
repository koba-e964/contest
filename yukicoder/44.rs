use std::cmp::*;
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

fn fib(n: i64) -> i64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0 .. n {
        let c = a + b;
        a = b;
        b = c;
    }
    return b
}

fn main() {
    let n = parse(getline().trim());
    println!("{}", fib(n));
}
