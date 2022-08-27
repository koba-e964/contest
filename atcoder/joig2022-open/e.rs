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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[derive(Copy, Clone, Debug)]
struct Max2<Kind, Value> {
    val: [(Value, Kind); 2],
}

impl<Kind: PartialEq + Clone, Value: PartialEq + PartialOrd + Clone> Max2<Kind, Value> {
    // a should be in the non-increasing order of Value.
    pub fn new(a: [(Value, Kind); 2]) -> Self {
        Max2 { val: a }
    }
    pub fn add(&mut self, value: Value, kind: Kind) {
        if self.val[1].0 >= value { return; }
        if self.val[0].1 == kind {
            if self.val[0].0 < value {
                self.val[0].0 = value
            }
            return;
        }
        if self.val[1].0 < value {
            self.val[1] = (value, kind)
        }
        if self.val[0].0 < self.val[1].0 {
            self.val.swap(0, 1);
        }
    }
    pub fn max_except(&self, kind: Kind) -> (Value, Kind) {
        if self.val[0].1 == kind {
            self.val[1].clone()
        } else {
            self.val[0].clone()
        }
    }
}

// Tags: two-biggest, max-2
fn main() {
    input! {
        n: usize,
        m: u32,
        av: [(u32, i64); n],
    }
    let mut max2 = Max2::new([(0, m + 1), (-1, m + 2)]);
    for (a, v) in av {
        let (ma, _) = max2.max_except(a);
        max2.add(ma + v, a);
    }
    println!("{}", max2.max_except(m + 2).0);
}
