#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
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
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

#[derive(Debug)]
enum Expr {
    Num(char),
    Op(char, Box<Expr>, Box<Expr>),
}

fn bfs(e: Expr) {
    let mut que = VecDeque::<&Expr>::new();
    let mut print_que = Vec::<char>::new();
    que.push_back(&e);
    while let Some(x) = que.pop_front() {
        match *x {
            Expr::Num(c) => print_que.push(c),
            Expr::Op(c, ref l, ref r) => {
                print_que.push(c);
                que.push_back(&l);
                que.push_back(&r);
            }
        }
    }
    print_que.reverse();
    for c in print_que {
        print!("{}", c);
    }
}

fn main() {
    let a: Vec<_> = get_word().chars().collect();
    let mut stack = Vec::new();
    for c in a {
        if '0' <= c && c <= '9' {
            stack.push(Expr::Num(c));
        } else {
            assert!(stack.len() >= 2);
            let r = stack.pop().unwrap();
            let l = stack.pop().unwrap();
            stack.push(Expr::Op(c, Box::new(l), Box::new(r)));
        }
    }
    assert!(stack.len() == 1);
    bfs(stack.remove(0));
    println!("");
}
