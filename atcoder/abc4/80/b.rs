fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, q] = ints[..] else { panic!() };
    let mut s = vec![];
    for _ in 0..n {
        s.push(getline().trim().bytes().collect::<Vec<_>>());
    }
    let f = |t: &[u8]| {
        for i in 0..n {
            if s[i] == t {
                return Some(i);
            }
        }
        let mut v = vec![];
        for i in 0..n {
            if s[i].len () != t.len() {
                continue;
            }
            let c = (0..t.len()).filter(|&j| s[i][j] != t[j]).count();
            if c == 1 {
                v.push(i);
            }
        }
        if v.len() == 1 {
            return Some(v[0]);
        }
        None
    };
    let mut v = vec![0; n];
    for _ in 0..q {
        let t = getline().trim().bytes().collect::<Vec<_>>();
        if let Some(idx) = f(&t) {
            v[idx] += 1;
        }
    }
    for i in 0..n {
        println!("{}", v[i]);
    }
}
