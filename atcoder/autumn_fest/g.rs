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

enum Expr {
    Number(u32),
    X,
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Call(u8, u32, Box<Expr>),
}

fn definition(x: &[u8]) -> (u8, Expr) {
    let name = x[0];
    let e = expr(&x[5..]).0;
    (name, e)
}

fn expr(s: &[u8]) -> (Expr, usize) {
    if s[0] == b'x' {
        return (Expr::X, 1);
    }
    if s[0] >= b'0' && s[0] <= b'9' {
        let (n, len) = number(s);
        return (Expr::Number(n), len);
    }
    if s[0] == b'(' {
        let (fst, len) = expr(&s[1..]);
        let op = s[1 + len];
        let (snd, len2) = expr(&s[2 + len..]);
        return (match op {
            b'|' => Expr::Or(Box::new(fst), Box::new(snd)),
            b'&' => Expr::And(Box::new(fst), Box::new(snd)),
            b'^' => Expr::Xor(Box::new(fst), Box::new(snd)),
            _ => panic!(),
        }, 3 + len + len2);
    }
    let name = s[0];
    let rep;
    let read;
    if s[1] == b'^' {
        let (n, len) = number(&s[2..]);
        rep = n;
        read = 2 + len + 1;
    } else {
        rep = 1;
        read = 2;
    }
    let (arg, len) = expr(&s[read ..]);
    (Expr::Call(name, rep, Box::new(arg)), read + len + 1)
}

fn number(s: &[u8]) -> (u32, usize) {
    let mut ptr = 0;
    while ptr < s.len() {
        if s[ptr] < b'0' || s[ptr] > b'9' {
            break;
        }
        ptr += 1;
    }
    (String::from_utf8(s[0 .. ptr].to_vec()).unwrap().parse().unwrap(), ptr)
}


fn eval(env: &HashMap<u8, Expr>, e: &Expr, b: usize, x: u32) -> u32 {
    use Expr::*;
    match *e {
        Number(u) => (u >> b) & 1,
        X => x,
        Or(ref e1, ref e2) => eval(env, e1, b, x) | eval(env, e2, b, x),
        And(ref e1, ref e2) => eval(env, e1, b, x) & eval(env, e2, b, x),
        Xor(ref e1, ref e2) => eval(env, e1, b, x) ^ eval(env, e2, b, x),
        Call(ref name, rep, ref e) => {
            let arg = eval(env, e, b, x);
            let body = env.get(name).unwrap();
            if rep == 0 {
                return arg;
            }
            let r0 = eval(env, body, b, 0);
            let r1 = eval(env, body, b, 1);
            match (r0, r1) {
                (0, 0) => 0,
                (1, 1) => 1,
                (0, 1) => arg,
                (1, 0) => if rep % 2 == 1 { 1 - arg } else { arg },
                _ => unreachable!(),
            }
        },
    }
}

fn solve() {
    let n: usize = get();
    let mut reg = HashMap::new();
    for _ in 0 .. n {
        let def: Vec<u8> = get_word().bytes().collect();
        let (name, expr) = definition(&def);
        // Goal: (x | param1) ^ param2
        let mut param1: u32 = 0;
        let mut param2: u32 = 0;
        for b in 0 .. 31 {
            let r0 = eval(&reg, &expr, b, 0);
            let r1 = eval(&reg, &expr, b, 1);
            let (add1, add2) = match (r0, r1) {
                (0, 0) => (1, 1),
                (0, 1) => (0, 0),
                (1, 0) => (0, 1),
                (1, 1) => (1, 0),
                _ => unreachable!(),
            };
            param1 |= add1 << b;
            param2 |= add2 << b;
        }
        println!("{}(x)=((x|{})^{})", name as char, param1, param2);
        reg.insert(name, expr);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
