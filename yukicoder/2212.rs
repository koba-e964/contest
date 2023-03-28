use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// https://yukicoder.me/problems/no/2212 (3)
// N = 1 のときは不可能。N >= 2 のときはサンプルで与えられている N = 2 の例から構築可能。
// N = k のとき構築できた行列を X として、N = k + 1 のときは (X, X + 2^{2k}; X + 2^{2k + 1}, X + 3 * 2^{2k}) でよい。
// -> この方法だと N >= 3 のとき xor が 0 になることが発覚。
// 4k, 4k+1, 4k+2, 4k+3 の xor が 0 になることを利用し、
// 左上の 4 * 4 行列以外を 4 * 4 のブロックに分け、それに 16k から 16k + 15 までの整数を順に埋めれば良い。
// -> これだと上 4 行、左 4 列だけしか xor が 1 にならないことが発覚。4 * 4 のブロックに分けた後、対角線部分のブロックの xor が 1 になるようにすればよい。
fn main() {
    let n: usize = get();
    if n == 1 {
        println!("-1");
        return;
    }
    let orig = vec![
        vec![7, 14, 0, 8],
        vec![4, 12, 2, 11],
        vec![15, 9, 6, 1],
        vec![13, 10, 5, 3],
    ];
    let mut dat = vec![vec![0; 1 << n]; 1 << n];
    for x in 0..1 << (n - 2) {
        for y in 0..1 << (n - 2) {
            for i in 0..4 {
                for j in 0..4 {
                    let val = if x == y {
                        orig[i][j]
                    } else {
                        4 * i + j
                    };
                    dat[4 * x + i][4 * y + j] = (x << (n - 2) | y) << 4 | val;
                }
            }
        }
    }
    for i in 0..1 << n {
        for j in 0..1 << n {
            print!("{}{}", dat[i][j], if j + 1 == 1 << n { "\n" } else { " " });
        }
    }
}
