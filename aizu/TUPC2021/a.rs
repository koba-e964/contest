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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    input! {
        m: usize,
        s1: String, s2: String,
        t: [(String, String); m],
    }
    let conv = |c: String| {
        let mut x = 0;
        for c in c.bytes() {
            x |= 1 << (c - b'a')
        }
        x
    };
    let s1 = conv(s1);
    let s2 = conv(s2);
    let t: Vec<_> = t.into_iter().map(|(t1, t2)| {
        (conv(t1), conv(t2))
    }).collect();
    for i in 0..26 {
        for j in 0..26 {
            if (s1 & 1 << i) != 0 && (s2 & 1 << j) != 0 {
                let mut ok = true;
                for k in 0..m {
                    let (t1, t2) = t[k];
                    if (t1 & 1 << i) != 0 && (t2 & 1 << j) != 0 {
                        ok = false;
                    }
                }
                if ok {
                    println!("Yes");
                    println!("{}{}", (b'a' + i) as char, (b'a' + j) as char);
                    return;
                }
            }
        }
    }
    println!("No");
}
