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

// Tags: graph-construction
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(a: usize, b: usize, c: usize, d: usize);
    if a + b - c > d + 2 || (a, b, c) == (1, 1, 1) {
        puts!("-1\n");
        return;
    }
    if a == c && b == c {
        assert!(c >= 2);
        if c > d {
            puts!("-1\n");
            return;
        }
        puts!("{} {}\n", c, c);
        for i in 0..c {
            puts!("{} {}\n", i, (i + 1) % c);
        }
        return;
    }
    if c == 0 {
        puts!("{} {}\n", a + b, a + b - 2);
        if a >= 2 {
            puts!("0 2\n");
            for i in 1..a - 1 {
                puts!("{} {}\n", i + 1, i + 2);
            }
        }
        if b >= 2 {
            puts!("1 {}\n", a + 1);
            for i in 1..b - 1 {
                puts!("{} {}\n", a + i, a + i + 1);
            }
        }
        return;
    }
    if a + b - c > d + 1 {
        puts!("-1\n");
        return;
    }
    puts!("{} {}\n", a + b - c, a + b - c - 1);
    let mut va = vec![];
    let mut vb = vec![];
    let mut vc = vec![];
    if a == c {
        vb.push(1);
        for i in 1..b - c {
            vb.push(1 + i);
        }
        vc.push(0);
        for i in 1..c {
            vc.push(b - c + i);
        }
    } else if b == c {
        va.push(0);
        for i in 1..a - c {
            va.push(1 + i);
        }
        vc.push(1);
        for i in 1..c {
            vc.push(a - c + i);
        }
    } else {
        va.push(0);
        vb.push(1);
        for i in 1..a - c {
            va.push(1 + i);
        }
        for i in 1..b - c {
            vb.push(a - c + i);
        }
        for i in 0..c {
            vc.push(a + b - 2 * c + i);
        }
    }
    va.push(vc[0]);
    vb.push(vc[0]);
    for i in 0..va.len() - 1 {
        puts!("{} {}\n", va[i], va[i + 1]);
    }
    for i in 0..vb.len() - 1 {
        puts!("{} {}\n", vb[i], vb[i + 1]);
    }
    for i in 0..vc.len() - 1 {
        puts!("{} {}\n", vc[i], vc[i + 1]);
    }
}
