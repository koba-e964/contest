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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize, _n: usize,
        t: chars,
        s: [chars; h],
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' { continue; }
            let mut ok = true;
            let mut x = i;
            let mut y = j;
            for &c in &t {
                let (nx, ny) = match c {
                    'U' => (x.wrapping_sub(1), y),
                    'D' => (x + 1, y),
                    'L' => (x, y.wrapping_sub(1)),
                    'R' => (x, y + 1),
                    _ => panic!(),
                };
                if s[nx][ny] == '#' {
                    ok = false;
                    break;
                }
                x = nx;
                y = ny;
            }
            if ok {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
