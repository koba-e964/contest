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

trait Int {
    fn ask(&mut self, idx: usize) -> i64 {
        println!("? {}", idx + 1);
        let x = get();
        x
    }
    fn ans(&mut self, a: i64) {
        println!("! {}", a);
    }
}

struct Actual;
impl Int for Actual {}

fn calc(int: &mut dyn Int, n: usize) {
    let mut fib = vec![0; 16];
    fib[0] = 1;
    fib[1] = 2;
    for i in 2..16 {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    let mut l = 0;
    let mut r = fib[15];
    let mut mid1 = fib[13];
    let mut mid2 = fib[14];
    let mut v1 = -1;
    let mut v2 = -1;
    for _ in 0..14 {
        if v1 < 0 {
            v1 = if mid1 <= n {
                int.ask(mid1 - 1)
            } else {
                -((mid1 - n) as i64)
            };
        }
        if v2 < 0 {
            v2 = if mid2 <= n {
                int.ask(mid2 - 1)
            } else {
                -((mid2 - n) as i64)
            };
        }
        if v1 > v2 {
            r = mid2;
            v2 = v1;
            mid2 = mid1;
            mid1 = l + r - mid2;
            v1 = -1;
        } else {
            l = mid1;
            v1 = v2;
            mid1 = mid2;
            mid2 = l + r - mid1;
            v2 = -1;
        }
    }
    assert_eq!(l + 1, mid1);
    int.ans(max(v1, v2));
}

fn test(n: usize, idx: usize) {
    let mut a = vec![0; n];
    let ma = 100_000;
    for i in 0..idx {
        a[i] = ma + i as i64 - idx as i64;
    }
    for i in idx..n {
        a[i] = ma + idx as i64 - i as i64;
    }
    struct Mock {
        a: Vec<i64>,
        idx: usize,
        count: usize,
    }
    let mut int = Mock {
        a: a,
        idx: idx,
        count: 0,
    };
    impl Int for Mock {
        fn ask(&mut self, idx: usize) -> i64 {
            self.count += 1;
            self.a[idx]
        }
        fn ans(&mut self, a: i64) {
            assert_eq!(self.a[self.idx], a);
            assert!(self.count <= 15);
        }
    }
    calc(&mut int, n);
}

fn main() {
    let n = 1500;
    for i in 0..n {
        test(n, i);
    }
    let n = 55;
    for i in 0..n {
        test(n, i);
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        calc(&mut Actual, n);
    }
}
