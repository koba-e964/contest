#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

#[derive(Clone, Copy, Eq, PartialEq)]
struct Date {
    day: i64, // #days since 1/1/1
}

impl Date {
    fn parse(s: &str) -> Self {
        let s: Vec<_> = s.split("/").collect();
        assert_eq!(s.len(), 3);
        let y: i64 = s[0].parse().unwrap();
        let m: usize = s[1].parse().unwrap();
        let d: i64 = s[2].parse().unwrap();
        let mut months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if (y % 400 == 0) ^ (y % 100 == 0) ^ (y % 4 == 0) {
            months[1] = 29;
        }
        let mut d = d - 1;
        for i in 0..m - 1 {
            d += months[i];
        }
        d += 365 * (y - 1) + (y - 1) / 400 + (y - 1) / 4 - (y - 1) / 100;
        Date {
            day: d,
        }
    }
    fn to_s(&self) -> String {
        let day = self.day;
        let per = day / (400 * 365 + 97);
        let rem = day % (400 * 365 + 97);
        let mut pass = 0;
        let mut fail = 400;
        while fail - pass > 1 {
            let mid = (fail + pass) / 2;
            if rem >= 365 * mid + mid / 4 - mid / 100 {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let y = 400 * per + pass + 1;
        let mut rem = rem - 365 * pass - pass / 4 + pass / 100;
        let mut months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if (y % 400 == 0) ^ (y % 100 == 0) ^ (y % 4 == 0) {
            months[1] = 29;
        }
        let mut m = 0;
        while m < 12 {
            if rem >= months[m] {
                rem -= months[m];
                m += 1;
            } else {
                break;
            }
        }
        format!("{}/{:02}/{:02}", y, m + 1, rem + 1)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Time {
    sec: i64,
}

impl Time {
    fn parse(s: &str) -> Time {
        let s: Vec<_> = s.split(":").collect();
        assert_eq!(s.len(), 3);
        let h: i64 = s[0].parse().unwrap();
        let m: i64 = s[1].parse().unwrap();
        let s: i64 = s[2].parse().unwrap();
        Time {
            sec: 3600 * h + 60 * m + s,
        }
    }
    fn to_s(&self) -> String {
        let sec = self.sec;
        let h = sec / 3600;
        let m = sec / 60 % 60;
        let s = sec % 60;
        format!("{:02}:{:02}:{:02}", h, m, s)
    }
}

// Tags: calendar
fn solve() {
    loop {
        let s = get_word();
        if s == "0" {
            break;
        }
        let t = get_word();
        let mut t = Time::parse(&t);
        let u = get_word().len();
        let u: i64 = (1 << u) - 1;
        let mut s = Date::parse(&s);
        t.sec += u;
        let q = t.sec / 86400;
        t.sec %= 86400;
        s.day += q;
        println!("{} {}", s.to_s(), t.to_s());
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
