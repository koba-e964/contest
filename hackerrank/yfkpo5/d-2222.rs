use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input!(n: usize);
    const INF: usize = 1 << 28;
    let mut dist = vec![(INF, vec!['}']); 44];
    dist[n] = (0, vec![]);
    for _ in 0..100 {
        for v in 1..44 {
            let mut me = dist[v].clone();
            let mut nxt = vec![];
            {
                let mut tmp = v * 2;
                if tmp >= 22 {
                    tmp = (tmp - 22) % 22 + 22;
                }
                nxt.push(('A', tmp));
            }
            if v >= 22 {
                nxt.push(('C', v % 22));
            }
            for &(c, w) in &nxt {
                let mut nxtd = dist[w].1.clone();
                nxtd.insert(0, c);
                let tmp = (dist[w].0 + 1, nxtd);
                if me > tmp {
                    me = tmp;
                }
            }
            if v >= 22 {
                let mut tmp = v % 22;
                let mut cnt = 0;
                while tmp > 0 && tmp % 2 == 0 {
                    tmp /= 2;
                    let mut nxtd = dist[tmp].1.clone();
                    nxtd.insert(0, 'C');
                    cnt += 1;
                    let nxtlen = dist[tmp].0 + cnt + 1;
                    for _ in 0..cnt {
                        nxtd.insert(1, 'B');
                    }
                    if me > (nxtlen, nxtd.clone()) {
                        me = (nxtlen, nxtd);
                    }
                }
            }
            dist[v] = me;
        }
    }
    if dist[1].0 >= INF {
        println!("-1");
        return;
    }
    println!("{}", dist[1].1.iter().cloned().collect::<String>());
}
