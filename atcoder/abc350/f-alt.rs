use std::collections::*;
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// weighted union heuristics
fn merge(mut a: (VecDeque<char>, bool), b: (VecDeque<char>, bool)) -> (VecDeque<char>, bool) {
    if a.0.len() < b.0.len() {
        let res = merge((b.0, !b.1), (a.0, !a.1));
        return (res.0, !res.1);
    }
    let mut bcont: Vec<_> = b.0.into();
    if b.1 {
        bcont.reverse();
    }
    if a.1 {
        for b in bcont {
            a.0.push_front(b);
        }
    } else {
        for b in bcont {
            a.0.push_back(b);
        }
    }
    a
}

fn dfs(s: &[char], rev: bool) -> ((VecDeque<char>, bool), usize) {
    let mut pos = 0;
    let mut ans = (VecDeque::new(), false);
    while pos < s.len() {
        if s[pos] == ')' {
            break;
        }
        if s[pos] == '(' {
            let (mut sub, prog) = dfs(&s[pos + 1..], !rev);
            sub.1 ^= true;
            ans = merge(ans, sub);
            pos += prog + 2;
            continue;
        }
        let mut sub = (VecDeque::new(), false);
        sub.0.push_back(if rev { (s[pos] as u8 ^ 0x20) as char } else { s[pos] });
        ans = merge(ans, sub);
        pos += 1;
    }
    (ans, pos)
}

fn solve() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let ((res, rev), _) = dfs(&s, false);
    let mut res: Vec<_> = res.into();
    if rev {
        res.reverse();
    }
    println!("{}", res.into_iter().collect::<String>());
}
