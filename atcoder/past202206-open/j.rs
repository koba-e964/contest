use std::cmp::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Date {
    day: i64, // #days since 1/1/1
}

impl Date {
    fn parse(s: &str) -> Self {
        let s: Vec<_> = s.split("-").collect();
        assert_eq!(s.len(), 3);
        let y: i64 = s[0].parse().unwrap();
        let m: usize = s[1].parse().unwrap();
        let d: i64 = s[2].parse().unwrap();
        let mut months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if (y % 400 == 0) ^ (y % 100 == 0) ^ (y % 4 == 0) {
            months[1] = 29;
        }
        let mut d = d - 1;
        for i in 0..m - 1 { d += months[i]; }
        d += 365 * (y - 1) + (y - 1) / 400 + (y - 1) / 4 - (y - 1) / 100;
        Date { day: d }
    }
}

fn main() {
    let s = Date::parse(getline().trim());
    let t = Date::parse(getline().trim());
    let base = Date::parse("2021-12-31");
    let s = s.day - 1 - base.day;
    let t = t.day - base.day;
    let f = |x| x / 7 * 2 + min(2, x % 7);
    println!("{}", f(t) - f(s));
}
