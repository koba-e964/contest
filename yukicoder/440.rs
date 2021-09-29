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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

struct Data {
    cww: Vec<i64>,
    w: Vec<i64>,
    c: Vec<i64>,
    cw: Vec<i64>,
}

impl Data {
    fn with(s: &[char]) -> Self {
        let n = s.len();
        let mut cww = vec![0; n + 1];
        let mut c = vec![0; n + 1];
        let mut w = vec![0; n + 1];
        let mut cw = vec![0; n + 1];
        for i in 0..n {
            cww[i + 1] = cww[i];
            c[i + 1] = c[i];
            w[i + 1] = w[i];
            cw[i + 1] = cw[i];
            if s[i] == 'c' {
                c[i + 1] += 1;
            } else {
                w[i + 1] += 1;
                cw[i + 1] += c[i];
                cww[i + 1] += cw[i];
            }
        }
        Data {
            cww: cww,
            w: w,
            c: c,
            cw: cw,
        }
    }
    // Counts cww.
    fn query(&self, a: usize, b: usize) -> i64 {
        let mut ans = self.cww[b] - self.cww[a];
        let rw = self.w[b] - self.w[a];
        ans -= self.c[a] * rw * (rw - 1) / 2;
        ans -= self.cw[a] * rw;
        ans
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: usize, w: usize,
        s: [chars; h],
        q: usize,
        abcd: [(usize1, usize1, usize, usize); q],
    }
    let mut row = vec![];
    let mut row_rev = vec![];
    for i in 0..h {
        row.push(Data::with(&s[i]));
        let mut tmp = s[i].to_vec();
        tmp.reverse();
        row_rev.push(Data::with(&tmp));
    }
    let mut col = vec![];
    let mut col_rev = vec![];
    for i in 0..w {
        let mut tmp = vec!['+'; h];
        for j in 0..h {
            tmp[j] = s[j][i];
        }
        col.push(Data::with(&tmp));
        tmp.reverse();
        col_rev.push(Data::with(&tmp));
    }
    for (a, b, c, d) in abcd {
        let mut ans = 0;
        for i in a..c {
            ans += row[i].query(b, d);
            ans += row_rev[i].query(w - d, w - b);
        }
        for i in b..d {
            ans += col[i].query(a, c);
            ans += col_rev[i].query(h - c, h - a);
        }
        puts!("{}\n", ans);
    }
}
