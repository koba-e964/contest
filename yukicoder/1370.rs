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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn sat(
    m: usize, lt: &[(usize, usize)], ne: &[(usize, usize)],
) -> Option<Vec<usize>> {
    let mut g = vec![vec![]; m];
    let mut indeg = vec![0; m];
    for &(a, b) in lt {
        if a == b { return None; }
        g[a].push(b);
        indeg[b] += 1
    }
    for &(a, b) in ne {
        if a == b { return None; }
    }
    let mut ans = vec![0; m];
    let mut que = vec![];
    for i in 0..m {
        if indeg[i] == 0 {
            que.push(i);
        }
    }
    let mut num = 0;
    while let Some(v) = que.pop() {
        num += 1;
        ans[v] = num;
        for &w in &g[v] {
            indeg[w] -= 1;
            if indeg[w] == 0 {
                que.push(w);
            }
        }
    }
    if num == m {
        Some(ans)
    } else {
        None
    }
}

// Tags: topological-sort
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize,
        a: [usize1; n],
    }
    for init in 0..2 {
        let mut lt = vec![];
        let mut ne = vec![];
        for i in 0..n - 1 {
            if (i + init) % 2 == 0 {
                lt.push((a[i], a[i + 1]));
            } else {
                lt.push((a[i + 1], a[i]));
            }
            if i < n - 2 {
                ne.push((a[i], a[i + 2]));
            }
        }
        if let Some(sol) = sat(m, &lt, &ne) {
            puts!("Yes\n");
            putvec!(sol);
            return;
        }
    }
    puts!("No\n");
}
