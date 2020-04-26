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

fn fill_one(n: usize, rowidx: usize, srowidx: i32,
            t: &[i32], v: &[u64],
            b: u32, mat: &mut [Vec<usize>])
-> Result<(), ()> {
    let mut zero = vec![];
    let mut one = vec![];
    let mut indet = vec![];
    for j in 0..n {
        let mut acc = if t[j] == 0 { 1 } else { 0 };
        let mut que = 0;
        for i in 0..n {
            if t[j] == 0 && mat[i][j] != 0 {
                acc &= mat[i][j] - 1;
            }
            if t[j] == 1 && mat[i][j] != 0 {
                acc |= mat[i][j] - 1;
            }
            if mat[i][j] == 0 {
                que += 1;
            }
        }
        if acc != ((v[j] >> b) & 1) as usize {
            if que == 0 {
                return Err(());
            }
            let to = ((v[j] >> b) & 1) as usize;
            if to == 0 {
                zero.push(j);
            } else {
                one.push(j);
            }
        } else {
            if que != 0 {
                indet.push(j);
            }
        }
        assert!(que <= 1);
    }
    if srowidx == 0 {
        let mut acc = 1;
        for j in 0..n {
            if mat[rowidx][j] != 0 {
                acc &= mat[rowidx][j] - 1;
            }
        }
        if zero.len() + indet.len() >= 1 || acc == 0 {
            for &idx in &zero {
                mat[rowidx][idx] = 1;
            }
            for &idx in &indet {
                mat[rowidx][idx] = 1;
            }
            for &idx in &one {
                mat[rowidx][idx] = 2;
            }
        } else {
            return Err(());
        }
    } else {
        let mut acc = 0;
        for j in 0..n {
            if mat[rowidx][j] != 0 {
                acc |= mat[rowidx][j] - 1;
            }
        }
        if one.len() + indet.len() >= 1 || acc == 1 {
            for &idx in &zero {
                mat[rowidx][idx] = 1;
            }
            for &idx in &indet {
                mat[rowidx][idx] = 2;
            }
            for &idx in &one {
                mat[rowidx][idx] = 2;
            }
        } else {
            return Err(());
        }
    }
    Ok(())
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        s: [i32; n],
        t: [i32; n],
        u: [u64; n],
        v: [u64; n],
    }
    let mut ans = vec![vec![0u64; n]; n];
    for b in 0..64 {
        let mut mat = vec![vec![0; n]; n];
        let mut row = vec![];
        let mut col = vec![];
        for i in 0..n {
            let pop = ((u[i] >> b) & 1) as i32;
            if s[i] == 0 && pop == 1 {
                for j in 0..n {
                    mat[i][j] |= 2;
                }
            }
            if s[i] == 1 && pop == 0 {
                for j in 0..n {
                    mat[i][j] |= 1;
                }
            }
            if s[i] == pop {
                row.push(i);
            }
        }
        for i in 0..n {
            let pop = ((v[i] >> b) & 1) as i32;
            if t[i] == 0 && pop == 1 {
                for j in 0..n {
                    mat[j][i] |= 2;
                }
            }
            if t[i] == 1 && pop == 0 {
                for j in 0..n {
                    mat[j][i] |= 1;
                }
            }
            if t[i] == pop {
                col.push(i);
            }
        }
        for i in 0..n {
            for j in 0..n {
                if mat[i][j] == 3 {
                    puts!("-1\n");
                    return;
                }
            }
        }
        if row.len() >= 2 && col.len() >= 2 {
            for i in 0..row.len() {
                let x = row[i];
                for j in 0..col.len() {
                    let todo = (i + j) % 2 + 1;
                    let y = col[j];
                    assert_eq!(mat[x][y], 0);
                    mat[x][y] = todo;
                }
            }
        } else if row.len() == 1 {
            let rowidx = row[0];
            assert_eq!(s[rowidx], ((u[rowidx] >> b) & 1) as i32);
            if fill_one(n, rowidx, s[rowidx], &t, &v, b, &mut mat).is_err() {
                puts!("-1\n");
                return;
            }
        } else if col.len() == 1 {
            let colidx = col[0];
            assert_eq!(t[colidx], ((v[colidx] >> b) & 1) as i32);
            // trans
            for i in 0..n {
                for j in 0..i {
                    let val = mat[i][j];
                    mat[i][j] = mat[j][i];
                    mat[j][i] = val;
                }
            }
            if fill_one(n, colidx, t[colidx], &s, &u, b, &mut mat).is_err() {
                puts!("-1\n");
                return;
            }
            // trans^{-1}
            for i in 0..n {
                for j in 0..i {
                    let val = mat[i][j];
                    mat[i][j] = mat[j][i];
                    mat[j][i] = val;
                }
            }
        }
        // verify
        for i in 0..n {
            let mut acc = if s[i] == 0 { 1 } else { 0 };
            for j in 0..n {
                assert!(mat[i][j] >= 1);
                assert!(mat[i][j] <= 2);
                if s[i] == 0 {
                    acc &= mat[i][j] - 1;
                } else {
                    acc |= mat[i][j] - 1;
                }
            }
            if ((u[i] >> b) & 1) as usize != acc {
                puts!("-1\n");
                return;
            }
        }
        for i in 0..n {
            let mut acc = if t[i] == 0 { 1 } else { 0 };
            for j in 0..n {
                if t[i] == 0 {
                    acc &= mat[j][i] - 1;
                } else {
                    acc |= mat[j][i] - 1;
                }
            }
            if ((v[i] >> b) & 1) as usize != acc {
                puts!("-1\n");
                return;
            }
        }
        for i in 0..n {
            for j in 0..n {
                if mat[i][j] == 2 {
                    ans[i][j] |= 1 << b;
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            puts!("{}{}", ans[i][j], if j + 1 == n { "\n" } else { " " });
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
