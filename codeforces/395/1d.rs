#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

#[derive(PartialEq,Eq,Hash,Clone,Copy,Debug)]
struct Hash {
    h: [i64; 2],
}
const MD: [i64; 2] = [1_000_000_007, 1_000_000_009];

impl Hash {
    fn new() -> Self { Hash::from(0) }
    fn from(v: i64) -> Self {
        Hash { h: [(v % MD[0] + MD[0]) % MD[0],
                   (v % MD[1] + MD[1]) % MD[1]] }
    }
}
impl std::ops::Add for Hash {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut ret = Self::new();
        for i in 0 .. 2 {
            ret.h[i] = (self.h[i] + other.h[i]) % MD[i];
        }
        ret
    }
}
impl std::ops::Neg for Hash {
    type Output = Self;
    fn neg(self) -> Self {
        let mut ret = Self::new();
        for i in 0 .. 2 {
            ret.h[i] = (MD[i] - self.h[i]) % MD[i];
        }
        ret
    }
}
impl std::ops::Mul for Hash {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut ret = Self::new();
        for i in 0 .. 2 {
            ret.h[i] = (self.h[i] * other.h[i]) % MD[i];
        }
        ret
    }
}

fn dfs(v: usize, p: Option<usize>, edges: &[Vec<usize>],
       dp: &mut [Vec<Hash>]) -> Hash {
    let mut children = Vec::new();
    for i in 0 .. edges[v].len() {
        let w = edges[v][i];
        if Some(w) == p { continue; }
        let h = dfs(w, Some(v), edges, dp);
        dp[v][i] = h;
        children.push(h);
    }
    let mut ret = Hash::from(135791357913579);
    for h in children {
        ret = ret + h * h;
    }
    ret
}
fn dfs_remain(v: usize, p: Option<usize>, r: Hash, edges: &[Vec<usize>],
              dp: &mut [Vec<Hash>]) {
    for i in 0 .. edges[v].len() {
        if Some(edges[v][i]) == p {
            dp[v][i] = r;
            break;
        }
    }
    let mut acc = Hash::from(135791357913579);
    for &h in dp[v].iter() {
        acc = acc + h * h;
    }
    for i in 0 .. edges[v].len() {
        let w = edges[v][i];
        if Some(w) == p { continue; }
        let subt = acc + (- dp[v][i] * dp[v][i]);
        dfs_remain(w, Some(v), subt, edges, dp);
    }
}

fn add_map(m: &mut HashMap<Hash, i32>, h: Hash, v: i32) {
    if let Some(&r) = m.get(&h) {
        if r + v == 0 {
            m.remove(&h);
        } else {
            assert!(r + v > 0);
            m.insert(h, r + v);
        }
    } else {
        m.insert(h, v);
    }
}

fn dfs_collect(v: usize, p: Option<usize>, m: &mut HashMap<Hash, i32>,
               edges: &[Vec<usize>], dp: &[Vec<Hash>]) {
    for i in 0 .. edges[v].len() {
        let w = edges[v][i];
        if Some(w) == p { continue; }
        add_map(m, dp[v][i], 1);
        dfs_collect(w, Some(v), m, edges, dp);
    }
}

fn dfs_exchange(v: usize, p: Option<usize>, m: &mut HashMap<Hash, i32>,
               mamaxi: &mut (usize, usize),
                edgesrevdp: &(&[Vec<usize>], &[Vec<usize>], &[Vec<Hash>])) {
    //println!("dfs_ex({}): {:?}", v, m);
    let edges = edgesrevdp.0;
    let rev = edgesrevdp.1;
    let dp = edgesrevdp.2;
    if mamaxi.0 < m.len() {
        mamaxi.0 = m.len();
        mamaxi.1 = v;
    }
    for i in 0 .. edges[v].len() {
        let w = edges[v][i];
        if Some(w) == p { continue; }
        // move to w, add dp[w][rev[v][i]] from m and erase dp[v][i] to m
        add_map(m, dp[v][i], -1);
        add_map(m, dp[w][rev[v][i]], 1);
        dfs_exchange(w, Some(v), m, mamaxi, edgesrevdp);
        add_map(m, dp[v][i], 1);
        add_map(m, dp[w][rev[v][i]], -1);
    }
}
/**
 * Reference: http://codeforces.com/contest/763/submission/24384757
 *              by uwi
 */

fn main() {
    let n = get();
    let mut edges = vec![Vec::new(); n];
    let mut rev = vec![Vec::new(); n];
    let mut dp = vec![Vec::new(); n];
    for _ in 0 .. n - 1 {
        let a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        edges[a].push(b);
        edges[b].push(a);
        rev[a].push(edges[b].len() - 1);
        rev[b].push(edges[a].len() - 1);
        dp[a].push(Hash::new());
        dp[b].push(Hash::new());
    }
    dfs(0, None, &edges, &mut dp);
    dfs_remain(0, None, Hash::new(), &edges, &mut dp);

    let mut m = HashMap::new();
    dfs_collect(0, None, &mut m, &edges, &dp);
    let ma = m.len();
    let maxi = 0;
    let mut mamaxi = (ma, maxi);
    dfs_exchange(0, None, &mut m, &mut mamaxi,
                 &(&edges, &rev, &dp));
    println!("{}", mamaxi.1 + 1);
}
