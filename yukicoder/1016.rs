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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1016 (3)
// 初手で勝てる場合は O の勝ち。
// 初手で勝てず、X の作り方が 2 通り以上ある場合は X の勝ち。
// そうでない場合、1 ターンではリーチを最大 1 個しか作ることができず、相手はそれを潰せば良いので、引き分け。
// -> ルールを誤解していた。O が勝てない場合は X の勝ちなので、初手で O が勝てなければ X の勝ち。
// -> ルールを誤解していた。o が 3 つ並んでも最後まで続くので、初期盤面で O の勝ちが決まっていることもある。また、-o– の場合、-oo- にすれば O が勝つ。
// -> o—o でも o-o-o にすれば O の勝ち。
// -> o–...--o (- が偶数個) でも O の勝ち。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        ns: [(usize, chars); t],
    }
    for (n, s) in ns {
        let mut t = vec![0; n];
        for i in 0..n {
            t[i] = match s[i] {
                'o' => 1,
                'x' => 2,
                _ => 0,
            };
        }
        let mut res = 0;
        for i in 0..std::cmp::max(2, n) - 2 {
            let mut f = [0; 3];
            for j in 0..3 {
                f[t[i + j]] += 1;
            }
            if f == [1, 2, 0] || f == [0, 3, 0] {
                res = 1;
            }
        }
        for i in 0..std::cmp::max(3, n) - 3 {
            let mut f = [0; 3];
            for j in 0..4 {
                f[t[i + j]] += 1;
            }
            if f == [3, 1, 0] && t[i] == 0 && t[i + 3] == 0 {
                res = 1;
            }
        }
        let mut v = vec![];
        for i in 0..n {
            if t[i] == 0 { continue; }
            if t[i] == 2 {
                v.clear();
                continue;
            }
            v.push(i);
            while v.len() > 2 {
                v.remove(0);
            }
            if v.len() >= 2 && (v[1] - v[0]) % 2 == 0 {
                res = 1;
            }
        }
        puts!("{}\n", ["X", "O"][res]);
    }
}
