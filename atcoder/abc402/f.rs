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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

fn main() {
    input! {
        n: usize, m: i64,
        a: [[i64; n]; n],
    }
    let mut fst = vec![vec![]; n];
    let mut bias = 1i64;
    for _ in 0..n - 1 {
        bias = bias * 10 % m;
    }
    for bits in 0..1 << (n - 1) {
        let mut cur = a[0][0] % m;
        let mut x = 0;
        let mut y = 0;
        for i in 0..n - 1 {
            if (bits & 1 << i) != 0 {
                x += 1;
            } else {
                y += 1;
            }
            cur = (10 * cur + a[x][y]) % m;
        }
        fst[x].push(cur * bias % m);
    }
    let mut snd = vec![vec![]; n];
    for bits in 0..1 << (n - 1) {
        let mut cur = 0;
        let mut base = 1;
        let mut x = n - 1;
        let mut y = n - 1;
        for i in 0..n - 1 {
            cur = (cur + a[x][y] * base) % m;
            base = base * 10 % m;
            if (bits & 1 << i) != 0 {
                x -= 1;
            } else {
                y -= 1;
            }
        }
        snd[x].push(cur);
    }
    let mut ans = 0;
    for i in 0..n {
        snd[i].sort_unstable();
        for &f in &fst[i] {
            let idx = snd[i].lower_bound(&(m - f));
            if idx > 0 {
                ans = ans.max(f + snd[i][idx - 1]);
            }
            // corner: should always be picked
            if !snd[i].is_empty() {
                ans = ans.max((f + snd[i][snd[i].len() - 1]) % m);
            }
        }
    }
    println!("{ans}");
}
