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

// https://yukicoder.me/problems/no/1717 (3)
// 操作後は 1 と 2 が隣り合うことはない。よって、1 回操作した後の列は .. 1 0 2 0 1 .. と 0 の何個かの連接である。 .. 1 0 2 0 1 .. の 1 個のみで成り立つ列であれば答えは中央の値で、そうでなければ 0 である。(操作している途中で .. 1 0 2 0 1 .. は長さが 2 減るため、長さが 2N + 1 未満であれば途中で消える。)
// -> 答えが 0 以外の場合、一番左の値は操作ごとに 1 と 2 を行き来する。よって n の偶奇によって場合分けすればよい。
fn main() {
    input! {
        n: usize,
        a: [usize; 2 * n + 1],
    }
    let mut b = vec![0; 2 * n - 1];
    for i in 0..2 * n - 1 {
        if (a[i] + a[i + 2] + a[i + 1]) % 3 == 0 {
            b[i] = (a[i + 1] + 2 * a[i]) % 3;
        } 
    }
    if b[0] == 0 {
        println!("0");
        return;
    }
    for i in 0..2 * n - 1 {
        let t = if i % 2 == 1 {
            0
        } else if i % 4 == 0 {
            b[0]
        } else {
            3 - b[0]
        };
        if b[i] != t {
            println!("0");
            return;
        }
    }
    if n % 2 == 1 {
        println!("{}", b[0]);
    } else {
        println!("{}", 3 - b[0]);
    }
}
