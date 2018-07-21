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
    let mut stdin = std::io::stdin();
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

fn operate(a: &[bool; 256], b: &[bool; 256], op: char) -> Option<[bool; 256]> {
    let mut ans = [false; 256];
    macro_rules! exhaust {
        ($op:expr) => {
            for i in 0 .. 256 {
                if a[i] {
                    for j in 0 .. 256 {
                        if b[j] {
                            ans[$op(i, j)] = true;
                        }
                    }
                }
            }
        };
    }
    match op {
        '+' => exhaust!(|i, j| (i + j) % 256),
        '-' => exhaust!(|i, j| (i + 256 - j) % 256),
        '*' => exhaust!(|i, j| (i * j) % 256),
        '/' => {
            if b[0] {
                return None;
            }
            exhaust!(|i, j| i / j);
        },
        _ => panic!(),
    }
    Some(ans)
}

fn solve() {
    let m = get();
    let mut hm = HashMap::new();
    for _ in 0 .. m {
        let name = get_word();
        let lb: usize = get();
        let ub: usize = get();
        hm.insert(name, (lb, ub));
    }
    let n = get();
    let e: Vec<String> = (0 .. n).map(|_| get_word()).collect();
    let mut stack = Vec::new();
    let mut mess = false;
    for e in &e {
        match e.parse::<usize>() {
            Ok(num) => {
                let mut t = [false; 256];
                t[num % 256] = true;
                stack.push(t);
            },
            Err(_) => {
                match hm.get(e) {
                    Some(&(lb, ub)) => {
                        let mut ans = [false; 256];
                        for i in lb .. ub + 1 {
                            ans[i] = true;
                        }
                        stack.push(ans);
                    },
                    None => {
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        if let Some(res) = operate(&a, &b, e.chars().nth(0).unwrap()) {
                            stack.push(res);
                        } else {
                            mess = true;
                            break;
                        }
                    },
                }
            }
        }
    }
    println!("{}", if mess { "error" } else { "correct" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
