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

fn main() {
    let line = getline();
    let s = line.trim().split(" ").collect::<Vec<_>>();
    let v: i32 = parse(&s[0][2..6]);
    let t: i32 = parse(s[1]);
    println!("{}", v * t / 10000);
}
