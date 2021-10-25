#[allow(unused_imports)]
use std::cmp::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn get_square_4(x: usize, y: usize, z: usize) -> Vec<Vec<usize>> {
    let a = [
        [5, 11, 6, 8],
        [14, 0, 13, 3],
        [9, 7, 10, 4],
        [2, 12, 1, 15],
    ];
    let mut nx = 0;
    let mut ny = 0;
    'outer:
    for i in 0..4 {
        for j in 0..4 {
            if a[i][j] == z {
                nx = (i + 4 - x) % 4;
                ny = (j + 4 - y) % 4;
                break 'outer;
            }
        }
    }
    let mut ans = vec![vec![0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            ans[i][j] = a[(i + nx) % 4][(j + ny) % 4];
        }
    }
    ans
}

fn get_square(n: usize, x: usize, y: usize, z: usize) -> Vec<Vec<usize>> {
    if n == 4 {
        return get_square_4(x, y, z);
    }
    let sub_z = z / 4;
    let sub = get_square(n / 2, x / 2, y / 2, sub_z);
    let mut flip_x = x % 2 == 1;
    let mut flip_y = y % 2 == 1;
    if z % 2 == 1 {
        flip_y ^= true;
    }
    if (z % 4) % 3 != 0 {
        flip_x ^= true;
    }
    flip_y ^= x >= n / 2;
    let mut ans = vec![vec![0; n]; n];
    for i in 0..n / 2 {
        for j in 0..n / 2 {
            let k = sub[i][j];
            let mut tmp = [
                [4 * k, 4 * k + 3],
                [4 * k + 2, 4 * k + 1],
            ];
            if flip_x {
                tmp.swap(0, 1);
            }
            if flip_y ^ (i >= n / 4) {
                for a in 0..2 {
                    tmp[a].swap(0, 1);
                }
            }
            for a in 0..2 {
                for b in 0..2 {
                    ans[2 * i + a][2 * j + b] = tmp[a][b];
                }
            }
        }
    }
    assert_eq!(ans[x][y], z);
    ans
}

fn main() {
    let n: usize = get();
    let y = get::<usize>() - 1;
    let x = get::<usize>() - 1;
    let z = get::<usize>() - 1;
    let ans = get_square(n, x, y, z);
    for i in 0..n {
        for j in 0..n {
            print!("{}{}", ans[i][j] + 1, if j + 1 == n { "\n" } else { " " });
        }
    }
}
