fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn walk(s: &[char], start: usize, end: usize, to: &[usize], rev: bool, ans: &mut String) {
    if start == end {
        return;
    }
    if !rev {
        for i in start..end {
            if s[i] == '(' {
                let j = to[i];
                walk(s, i + 1, j, to, true, ans);
                walk(s, j + 1, end, to, false, ans);
                return;
            } else {
                ans.push(s[i]);
            }
        }
    } else {
        for i in (start..end).rev() {
            if s[i] == ')' {
                let j = to[i];
                walk(s, j + 1, i, to, false, ans);
                walk(s, start, j, to, true, ans);
                return;
            } else {
                let conv = if s[i].is_ascii_uppercase() {
                    s[i].to_ascii_lowercase()
                } else {
                    s[i].to_ascii_uppercase()
                };
                ans.push(conv);
            }
        }
    }
}

fn solve() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut to = vec![0; n];
    let mut st = vec![];
    for i in 0..n {
        let c = s[i];
        if c == '(' {
            st.push(i);
        }
        if c == ')' {
            let j = st.pop().unwrap();
            to[i] = j;
            to[j] = i;
        }
    }
    let mut ans = "".to_string();
    walk(&s, 0, n, &to, false, &mut ans);
    println!("{}", ans);
}
