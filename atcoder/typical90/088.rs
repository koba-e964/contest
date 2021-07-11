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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn dfs(v: usize, g: &[Vec<bool>], a: &[usize],
       sum: usize, picked: &mut [bool],
       memo: &mut [Option<Vec<bool>>]) -> Result<(), [Vec<bool>; 2]> {
    let n = g.len();
    if v >= n {
        if let &Some(ref oth) = &memo[sum] {
            return Err([picked.to_vec(), oth.to_vec()]);
        } else {
            memo[sum] = Some(picked.to_vec());
            return Ok(());
        }
    }
    dfs(v + 1, g, a, sum, picked, memo)?;
    let mut ok = true;
    for i in 0..n {
        if g[i][v] && picked[i] {
            ok = false;
            break;
        }
    }
    if ok {
        picked[v] = true;
        dfs(v + 1, g, a, sum + a[v], picked, memo)?;
        picked[v] = false;
    }
    Ok(())
}

// Tags: knapsack, pigeonhole-principle
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
        n: usize, q: usize,
        a: [usize; n],
        xy: [(usize1, usize1); q],
    }
    let mut g = vec![vec![false; n]; n];
    for &(x, y) in &xy {
        g[x][y] = true;
        g[y][x] = true;
    }
    let mut memo = vec![None; 8889];
    let sub = dfs(0, &g, &a, 0, &mut vec![false; n], &mut memo).unwrap_err();
    fn conv(a: &[bool]) -> Vec<usize> {
        let mut ans = vec![];
        for i in 0..a.len() {
            if a[i] {
                ans.push(i + 1);
            }
        }
        ans
    }
    let s = conv(&sub[0]);
    puts!("{}\n", s.len());
    putvec!(s);
    let s = conv(&sub[1]);
    puts!("{}\n", s.len());
    putvec!(s);
}
