use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

const RADIX: i32 = 10;

fn mulprec_add(a: &mut [i32], mut x: i32) {
    for a in a.iter_mut().rev() {
        let tmp = *a + x;
        let mut r = tmp % RADIX;
        if r < 0 {
            r += RADIX;
        }
        *a = r;
        x = (tmp - r) / RADIX;
    }
}

fn mulprec_div(a: &mut [i32], x: i32) -> i32 {
    let mut c = 0;
    for a in a {
        c = RADIX * c + *a;
        *a = c / x;
        c %= x;
    }
    c
}

// Tags: multi-precision
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(x: String);
    let mut x: Vec<_> = x.chars().map(|x| (x as u8 - b'0') as i32).collect();
    x.push(0);
    let s: i32 = x.iter().sum();
    mulprec_add(&mut x, -s);
    let r = mulprec_div(&mut x, 9);
    assert_eq!(r, 0);
    let mut seen = false;
    for x in x {
        if x == 0 {
            if seen {
                puts!("0");
            }
        } else {
            puts!("{}", x);
            seen = true;
        }
    }
    puts!("\n");
}
