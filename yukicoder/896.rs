#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        m: usize, n: usize, mulx: i32, addx: i32, muly: i64, addy: i64,
        mo: i32,
        x: [i32; m],
        y: [i64; m],
        a: [i32; m],
        b: [i64; m],
    }
    let mut pr = vec![true; mo as usize];
    pr[0] = false;
    pr[1] = false;
    for i in 2..mo as usize {
        if !pr[i] { continue; }
        for j in 2..(mo as usize - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    let mut z = vec![0i64; mo as usize];
    for i in 0..m {
        z[x[i] as usize] += y[i] as i64;
    }
    let mut xl = x[m - 1];
    let mut yl = y[m - 1];
    for _ in m..n {
        let nxl = xl.wrapping_mul(mulx).wrapping_add(addx) & (mo - 1);
        let nyl = yl.wrapping_mul(muly).wrapping_add(addy) & (mo as i64 - 1);
        z[nxl as usize] += nyl;
        xl = nxl;
        yl = nyl;
    }
    // debugln!("{:?}", z);
    for i in 2..mo as usize {
        if !pr[i] { continue; }
        for j in (1..(mo as usize - 1) / i + 1).rev() {
            z[j] += z[j * i];
        }
    }
    // debugln!("{:?}", z);
    let mut ans = vec![0; m];
    let mut anssum = 0;
    
    for i in 0..m {
        if a[i] < mo {
            ans[i] += z[a[i] as usize];
        }
        if a[i] as i64 * b[i] < mo as i64 {
            ans[i] -= z[(a[i] as i64 * b[i]) as usize];
        }
        anssum ^= ans[i];
    }
    let mut al = a[m - 1];
    let mut bl = b[m - 1];
    for _ in m..n {
        al = (al.wrapping_mul(mulx).wrapping_add(addx + mo - 1) & (mo - 1)) + 1;
        bl = (bl.wrapping_mul(muly).wrapping_add(addy + mo as i64 - 1) & (mo as i64 - 1)) + 1;
        let mut cur = 0;
        if al < mo {
            cur += z[al as usize];
        }
        if (al as i64 * bl) < (mo as i64) {
            cur -= z[(al as i64 * bl) as usize];
        }
        anssum ^= cur;
    }
    for i in 0..m {
        puts!("{}\n", ans[i]);
    }
    puts!("{}\n", anssum);
}
