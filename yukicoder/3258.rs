use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

#[derive(Debug)]
struct Stat {
    l: usize,
    r: usize,
    unique: BTreeSet<usize>, // index
    freq: HashMap<u32, BTreeSet<usize>>,
}

fn split(a: &[u32], mut s: Stat, idx: usize) -> (Stat, Stat) {
    let val = a[idx];
    assert_eq!(s.freq[&val].len(), 1);
    s.freq.remove(&val);
    s.unique.remove(&idx);
    let mut new = Stat {
        l: s.l,
        r: s.r,
        unique: BTreeSet::new(),
        freq: HashMap::new(),
    };
    if 2 * idx >= s.l + s.r {
        for i in idx + 1..s.r {
            let f = a[i];
            let entry = s.freq.get_mut(&f).unwrap();
            entry.remove(&i);
            if entry.is_empty() {
                s.unique.remove(&i);
            }
            if entry.len() == 1 {
                let &only = entry.iter().next().unwrap();
                s.unique.insert(only);
            }
            let entry = new.freq.entry(f).or_insert(BTreeSet::new());
            if entry.is_empty() {
                new.unique.insert(i);
            } else if entry.len() == 1 {
                let &only = entry.iter().next().unwrap();
                new.unique.remove(&only);
            }
            entry.insert(i);
        }
        new.l = idx + 1;
        s.r = idx;
        return (s, new);
    }
    for i in s.l..idx {
        let f = a[i];
        let entry = s.freq.get_mut(&f).unwrap();
        entry.remove(&i);
        if entry.is_empty() {
            s.unique.remove(&i);
        }
        if entry.len() == 1 {
            let &only = entry.iter().next().unwrap();
            s.unique.insert(only);
        }
        let entry = new.freq.entry(f).or_insert(BTreeSet::new());
        if entry.is_empty() {
            new.unique.insert(i);
        } else if entry.len() == 1 {
            let &only = entry.iter().next().unwrap();
            new.unique.remove(&only);
        }
        entry.insert(i);
    }
    new.r = idx;
    s.l = idx + 1;
    (new, s)
}

fn rec(a: &[u32], mut s: Stat) -> i32 {
    // eprintln!("stat = {s:?}");
    let mut ans = 0;
    while let Some(&idx) = s.unique.first() {
        let (t, tmp) = split(a, s, idx);
        ans += rec(a, t) + 1;
        s = tmp;
    }
    // eprintln!("stat = {s:?}, ans = {ans}");
    ans
}

// https://yukicoder.me/problems/no/3258 (3.5)
// alone な要素によって貪欲に分割していけば、最終的なゲームの結果が得られる。
// naïve に simulation を行うと O(N^2) かかる。
// マージテクの逆で、HashMap に頻度表 (値 => 個数) を持っておき、分割時に大きい方を保存することにする。
// (alone な要素は、BTreeSet で持ち、分割する時に操作することにする。)
// そうすれば全体で O(N log N) になるはず。
// -> alone な要素は index を持つ必要があり、分割するときの便宜のため頻度表は (値 => index の集合) とする必要がある。
// Tags: inverse-of-weighted-union-heuristics, divide-segments
fn main() {
    getline();
    let f = getline().trim().split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let n = f.len();
    let mut freq = HashMap::new();
    for i in 0..n {
        freq.entry(f[i]).or_insert(BTreeSet::new()).insert(i);
    }
    let stat = Stat {
        l: 0,
        r: n,
        unique: {
            let mut x = BTreeSet::new();
            for i in 0..n {
                if freq[&f[i]].len() == 1 {
                    x.insert(i);
                }
            }
            x
        },
        freq,
    };
    println!("{}", if rec(&f, stat) % 2 == 1 {
        "Alice"
    } else {
        "Bob"
    });
}
