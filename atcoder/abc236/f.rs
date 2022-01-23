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

#[derive(Clone, Default)]
struct BinaryMat {
    basis: Vec<i64>,
}

impl BinaryMat {
    // O(1)
    fn new() -> Self {
        Default::default()
    }
    // O(rank)
    fn sift(&self, mut x: i64) -> i64 {
        for &b in &self.basis {
            x = std::cmp::min(x, x ^ b);
            if x == 0 {
                return 0;
            }
        }
        x
    }
    // O(rank)
    fn add(&mut self, x: i64) {
        let x = self.sift(x);
        if x != 0 {
            self.basis.push(x);
        }
    }
    // O(1)
    #[allow(unused)]
    fn rank(&self) -> usize {
        self.basis.len()
    }
    // O(rank)
    #[allow(unused)]
    fn is_indep(&self, x: i64) -> bool {
        self.sift(x) != 0
    }
}

fn main() {
    input! {
        n: usize,
        c: [i64; (1 << n) - 1],
    }
    let mut ans = 0;
    let mut mat = BinaryMat::new();
    let mut e = vec![];
    for i in 1..1 << n {
        e.push((c[i - 1], i));
    }
    e.sort();
    for (w, v) in e {
        if mat.is_indep(v as i64) {
            mat.add(v as i64);
            ans += w;
        }
    }
    println!("{}", ans);
}
