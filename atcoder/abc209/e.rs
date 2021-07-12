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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: backward-analysis, topological-sort, game
fn main() {
    input! {
        n: usize,
        s: [chars; n],
    }
    const W: usize = 52 * 52 * 52;
    let to_idx = |c: &[char]| {
        let mut v = 0;
        for i in 0..3 {
            let idx = if c[i] >= 'a' {
                (c[i] as u8 - b'a') as usize + 26
            } else {
                (c[i] as u8 - b'A') as usize
            };
            v = 52 * v + idx;
        }
        v
    };
    let s: Vec<_> = s.iter().map(|s| (to_idx(s), to_idx(&s[s.len() - 3..])))
        .collect();
    // 0: lose
    // 1: win
    // 2: draw
    let mut sc = vec![2; W];
    let mut g = vec![vec![]; W];
    let mut win = vec![0; W];
    let mut lose = vec![0; W];
    let mut tot = vec![0; W];
    for &(a, b) in &s {
        g[b].push(a);
        tot[a] += 1;
    }
    let mut st = vec![];
    for i in 0..W {
        if tot[i] == 0 {
            st.push(i);
        }
    }
    let mut seen = vec![false; W];
    while let Some(v) = st.pop() {
        assert!(!seen[v]);
        seen[v] = true;
        if lose[v] == 0 {
            sc[v] = 0;
            for &w in &g[v] {
                lose[w] += 1;
                if lose[w] == 1 {
                    st.push(w);
                }
            }
        } else {
            sc[v] = 1;
            for &w in &g[v] {
                win[w] += 1;
                if win[w] == tot[w] {
                    st.push(w);
                }
            }
        }
    }
    for i in 0..n {
        println!("{}", ["Takahashi", "Aoki", "Draw"][sc[s[i].1]]);
    }
}
