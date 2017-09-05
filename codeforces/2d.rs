#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
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
fn get<T: std::str::FromStr>() -> T {
    get_word().parse().ok().unwrap()
}


/*
 * Suffix Array by Manber & Myers.
 * Verified by: AtCoder ARC050 (http://arc050.contest.atcoder.jp/submissions/818912)
 * Reference: http://mayokoex.hatenablog.com/entry/2016/04/03/145845
 */
fn create_sa(s: &[char]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0 .. n + 1).collect();
    let mut rank: Vec<usize> = vec![0; n + 1];
    let mut tmp = vec![0; n + 1];

    for i in 0 .. n + 1 {
        rank[i] = if i < n { s[i] as usize + 1 } else { 0_usize };
    }
    let mut k = 1;
    while k <= n {
        {
            let key = |i: &usize| {
                let ri = if i + k <= n { rank[i + k] as i32 } else { -1 };
                (rank[*i], ri)
            };
            sa.sort_by_key(&key);
            tmp[sa[0]] = 0;
            for i in 1 .. n + 1 {
                tmp[sa[i]] = tmp[sa[i - 1]]
                    + if key(&sa[i - 1]) < key(&sa[i]) { 1 } else { 0 };
            }
        }
        rank.clone_from_slice(&tmp);
        k *= 2;
    }
    return sa;
}

struct LCP {
    inv_sa: Vec<usize>,
    spt: Vec<Vec<usize>>
}

impl LCP {
    pub fn new(s: &[char], sa: &[usize]) -> LCP {
        let n = sa.len() - 1;
        let mut inv_sa = vec![0; n + 1];
        for i in 0 .. n + 1 {
            inv_sa[sa[i]] = i;
        }
        let lcp = Self::create_lcp(s, sa);
        let spt = Self::create_sparse_table(&lcp);
        LCP {
            inv_sa: inv_sa,
            spt: spt,
        }
    }
    fn create_lcp(s: &[char], sa: &[usize]) -> Vec<usize> {
        let n = s.len();
        let mut rank = vec![0; n + 1];
        let mut lcp = vec![0; n];
        for i in 0 .. n + 1 {
            rank[sa[i]] = i;
        }
        let mut h: usize = 0;
        lcp[0] = 0;
        for i in 0 .. n {
            let j = sa[rank[i] - 1];
            h = h.saturating_sub(1);
            while j + h < n && i + h < n {
                if s[j + h] != s[i + h] {
                    break;
                }
                h += 1;
            }
            lcp[rank[i] - 1] = h;
        }
        return lcp;
    }
    
    fn create_sparse_table(lcp: &[usize]) -> Vec<Vec<usize>> {
        let n = lcp.len();
        let mut h: usize = 1;
        while (1 << h) <= n {
            h += 1;
        }
        let mut st: Vec<Vec<usize>> = vec![Vec::new(); h];
        st[0] = Vec::from(lcp);
        for j in 1 .. h {
            st[j] = vec![0; n + 1 - (1 << j)];
            for i in 0 .. n + 1 - (1 << j) {
                st[j][i] = std::cmp::min(
                    st[j - 1][i],
                    st[j - 1][i + 1_usize.wrapping_shl(j as u32 - 1)]);
            }
        }
        return st;
    }
    
    pub fn get_lcp(&self, f: usize, s: usize) -> usize {
        let f = self.inv_sa[f];
        let s = self.inv_sa[s];
        let (f, s) = 
            if f > s {
                (s, f)
            } else {
                (f, s)
            };
        assert!(f < s);
        let usize_size = usize::max_value().count_ones();
        let diff = usize_size - 1 - (s - f).leading_zeros(); // topmost 1
        return std::cmp::min(self.spt[diff as usize][f],
                             self.spt[diff as usize][s - 1_usize.wrapping_shl(diff)]);
    }
}


fn solve() {
    let mut s: Vec<char> = get_word().chars().collect();
    let n = s.len();
    {
        let mut revs = s.clone();
        revs.reverse();
        s.append(&mut revs);
    }
    let sa = create_sa(&s);
    let salcp = LCP::new(&s, &sa);
    let mut ans = vec![0; n + 1];
    let mut cand = vec![vec![0i16; n]; n];
    for s in 0 .. n {
        for i in 0 .. n - s {
            // is_pal?
            let j = i + s;
            let lcp_len = salcp.get_lcp(i, 2 * n - 1 - j);
            if lcp_len < j - i + 1 {
                continue;
            }
            let mut res = 1;
            if s >= 1 {
                let half = (s + 1) / 2;
                let sub = cand[i][i + half - 1];
                res = sub + 1;
            }
            cand[i][j] = res;
            ans[res as usize] += 1;
        }
    }
    for i in (1 .. n).rev() {
        ans[i] += ans[i + 1];
    }
    for i in 1 .. n + 1 {
        print!("{}{}", ans[i], if i == n { "\n" } else { " " });
    }
    
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
