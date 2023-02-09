use std::collections::*;
use std::io::{Write, BufWriter, Read};

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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Tags: persistent-data-structure
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    let q: usize = get();
    let mut a = vec![None];
    let mut tip = 0;
    let mut map = HashMap::new();
    let mut ans = vec![];
    for _ in 0..q {
        let ty: String = get_word();
        if ty == "ADD" {
            let x: i32 = get();
            a.push(Some((tip, x)));
            tip = a.len() - 1;
        }
        if ty == "DELETE" {
            if tip > 0 {
                tip = a[tip].unwrap().0;
            }
        }
        if ty == "SAVE" {
            let x: u32 = get();
            map.insert(x, tip);
        }
        if ty == "LOAD" {
            let x: u32 = get();
            tip = *map.get(&x).unwrap_or(&0);
        }
        if tip == 0 {
            ans.push(-1);
        } else {
            ans.push(a[tip].unwrap().1);
        }
    }
    putvec!(ans);
}
