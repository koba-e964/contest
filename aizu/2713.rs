#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word<R: std::io::Read>(br: &mut std::io::BufReader<R>) -> String {
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = br.read(&mut u8b);
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
fn get<R: std::io::Read, T: std::str::FromStr>(br: &mut std::io::BufReader<R>) -> T { get_word(br).parse().ok().unwrap() }

/**
 * Returns the least index of elements that are modified, wrapped with Some.
 * If the entire array is reversed, it returns None instead.
 * v's elements must be pairwise distinct.
 */
fn next_permutation<T: Ord>(v: &mut [T]) -> Option<usize> {
    let mut tail_dec: usize = 1;
    let n = v.len();
    while tail_dec < n {
        if v[n - tail_dec - 1] > v[n - tail_dec] {
            tail_dec += 1;
        } else {
            break;
        }
    }
    // v[n - tail_dec .. n] is strictly decreasing
    if tail_dec < n {
        let x = n - tail_dec - 1;
        let mut y = n;
        {
            let pivot = &v[x];
            for i in (n - tail_dec .. n).rev() {
                if v[i] > *pivot {
                    y = i;
                    break;
                }
            }
            assert!(y < n);
        }
        v.swap(x, y);
    }
    v[n - tail_dec .. n].reverse();
    if tail_dec < n {
        Some(n - tail_dec - 1)
    } else {
        None
    }
}

#[derive(Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy)]
enum Var { X, Y }

fn perform(val: usize, op: (Op, Var)) -> usize {
    let aff = match op.1 {
        Var::X => 6, // **.
        Var::Y => 5, // *.*
    };
    match op.0 {
        Op::And => val & aff,
        Op::Or => val | aff,
        Op::Xor => val ^ aff,
    }
}

fn dfs(v: usize, par: usize, maxim: bool, val: usize,
       g: &[Vec<usize>], inv: &[usize; 8],
       kind: &[(Op, Var)]) -> usize {
    let mut num_ch = 0;
    let mut ch = if maxim { (0, 0) } else { (100, 100) };
    for &w in &g[v] {
        if w == par { continue; }
        let next_val = perform(val, kind[w]);
        let sub = dfs(w, v, !maxim, next_val, g, inv, kind);
        ch = if maxim {
            max(ch, (inv[sub], sub))
        } else {
            min(ch, (inv[sub], sub))
        }; 
        num_ch += 1;
    }
    if num_ch == 0 {
        return val;
    }
    ch.1
}

fn solve() {
    let stdin = std::io::stdin();
    let mut br = std::io::BufReader::new(stdin.lock());
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n = get(&mut br);
    let m: usize = get(&mut br);
    let mut o: Vec<_> = (1 .. n).map(|_| get_word(&mut br)).collect();
    o.insert(0, "".to_string());
    let mut kind = vec![(Op::And, Var::X); n];
    for i in 1 .. n {
        let chs: Vec<char> = o[i].chars().collect();
        let op = match chs[3] {
            '&' => Op::And,
            '|' => Op::Or,
            '^' => Op::Xor,
            _ => panic!(),
        };
        let v = match chs[4] {
            'X' => Var::X,
            'Y' => Var::Y,
            _ => panic!(),
        };
        kind[i] = (op, v);
    }
    let mut g = vec![Vec::new(); n];
    for _ in 0 .. n - 1 {
        let u = get::<_, usize>(&mut br);
        let v = get::<_, usize>(&mut br);
        g[u].push(v);
        g[v].push(u);
    }
    // 6! brute force
    let mut perm = [0; 8];
    for i in 0 .. 8 {
        perm[i] = i;
    }
    let mut hm = HashMap::new();
    loop {
        let mut inv = [0; 8];
        for i in 0 .. 8 { inv[perm[i]] = i; }
        let mut ok = true;
        for i in 0 .. 8 {
            for j in i + 1 .. 8 {
                if (i & j) == i {
                    if inv[i] > inv[j] {
                        ok = false;
                        break;
                    }
                }
            }
        }
        if ok {
            let ans = dfs(0, n + 1, true, 0, &g, &inv, &kind);
            let mut key = [0; 6];
            key.copy_from_slice(&perm[1..7]);
            hm.insert(key, ans);
        }
        if let None = next_permutation(&mut perm[1..7]) { break; }
    }
    for _ in 0 .. m {
        let x: i32 = get(&mut br);
        let y: i32 = get(&mut br);
        let mut pool = Vec::new();
        pool.push((!x & y, 1));
        pool.push((x & !y, 2));
        pool.push((x ^ y, 3));
        pool.push((x & y, 4));
        pool.push((y, 5));
        pool.push((x, 6));
        pool.sort();
        let mut key = [0; 6];
        for i in 0 .. 6 { key[i] = pool[i].1; }
        let ans = hm.get(&key).unwrap();
        let tbl = [0, !x & y, x & !y, x ^ y, x & y, y, x, x | y];
        puts!("{}\n", tbl[*ans]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
