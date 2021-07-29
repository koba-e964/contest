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

// Tags: topological-sort, queue
fn main() {
    input! {
        h: usize, _w: usize,
        s: [chars; h],
    }
    let mut ans = 0;
    let mut que = get(&s);
    let mut s = s;
    fn get(s: &[Vec<char>]) -> Vec<(usize, usize)> {
        let h = s.len();
        let w = s[0].len();
        let mut ans = vec![];
        for i in 1..h - 1 {
            for j in 1..w - 1 {
                if s[i][j] == '.' {
                    continue;
                }
                let mut tmp = 0;
                for x in i - 1..i + 2 {
                    for y in j - 1..j + 2 {
                        if s[x][y] == '.' {
                            tmp += 1;
                        }
                    }
                }
                let c = (s[i][j] as u8 - b'0') as i32;
                if c <= tmp {
                    ans.push((i, j));
                }
            }
        }
        ans
    }
    while !que.is_empty() {
        ans += 1;
        let mut nxt = vec![];
        for (x, y) in que {
            s[x][y] = '.';
            for a in x - 1..x + 2 {
                for b in y - 1..y + 2 {
                    if s[a][b] != '.' {
                        let mut tmp = 0;
                        for c in a - 1..a + 2 {
                            for d in b - 1..b + 2 {
                                if s[c][d] == '.' {
                                    tmp += 1;
                                }
                            }
                        }
                        if tmp == (s[a][b] as u8 - b'0') as i32 {
                            nxt.push((a, b));
                        }
                    }
                }
            }
        }
        que = nxt;
    }
    println!("{}", ans);
}
