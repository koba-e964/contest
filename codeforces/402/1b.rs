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

enum Op {
    And,
    Or,
    Xor,
}


enum IrnBru {
    Imm(Vec<bool>),
    Bin(Op, Option<usize>, Option<usize>),
}

type Tank = Vec<IrnBru>;

fn calc(s: &Tank, k: usize, x: bool) -> usize {
    let n = s.len();
    let mut cnt = 0;
    let mut pool = vec![false; n];
    for i in 0 .. n {
        pool[i] = match s[i] {
            IrnBru::Imm(ref imm) => imm[k],
            IrnBru::Bin(ref op, ref u, ref v) => {
                let uu = match *u { Some(j) => pool[j], None => x, };
                let vv = match *v { Some(j) => pool[j], None => x, };
                match *op {
                    Op::And => uu && vv,
                    Op::Or => uu || vv,
                    Op::Xor => uu ^ vv,
                }
            },
        };
        if pool[i] {
            cnt += 1;
        }
    }
    cnt
}

// (min, max)
fn find(s: &Tank, k: usize) -> (bool, bool) {
    let x = calc(s, k, false);
    let y = calc(s, k, true);
    if x == y {
        (false, false)
    } else {
        (x > y, x < y)
    }
}

fn solve() {
    let first = getline();
    let sp = first.trim().split(" ").collect::<Vec<_>>();
    let n = sp[0].parse().ok().unwrap();
    let m: usize = sp[1].parse().ok().unwrap();
    let mut reg_inv = HashMap::new();
    let mut irn = Vec::new();
    for _ in 0 .. n {
        let line = getline();
        let s: Vec<_> = line.trim().split(" ").collect();
        let vn = s[0].to_string();
        assert_eq!(s[1], ":=");
        let sz = reg_inv.len();
        reg_inv.insert(vn, sz);
        if s.len() == 5 { // bin
            let op;
            if s[3] == "OR" {
                op = Op::Or;
            } else if s[3] == "AND" {
                op = Op::And;
            } else {
                op = Op::Xor;
            }
            irn.push(IrnBru::Bin(op,
                                 reg_inv.get(s[2]).cloned(),
                                 reg_inv.get(s[4]).cloned()));
        } else {
            assert_eq!(s.len(), 3);
            let seq: Vec<bool> = s[2].bytes().map(|x| x == b'1').collect();
            assert_eq!(seq.len(), m);
            irn.push(IrnBru::Imm(seq));
        }
    }
    let mut mi = "".to_string();
    let mut ma = "".to_string();
    for k in 0 .. m {
        let (a, b) = find(&irn, k);
        mi.push(if a { '1' } else { '0' });
        ma.push(if b { '1' } else { '0' });
    }
    println!("{}", mi);
    println!("{}", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
