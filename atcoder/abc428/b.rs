fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let k = ints[1];
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut hm = std::collections::HashMap::new();
    for i in 0..n - k + 1 {
        *hm.entry(s[i..i + k].to_vec()).or_insert(0) += 1;
    }
    let mut ma = 0;
    for (_, &v) in &hm {
        ma = ma.max(v);
    }
    println!("{ma}");
    let mut ans = vec![];
    for (k, v) in hm {
        if v == ma {
            ans.push(k.into_iter().collect::<String>());
        }
    }
    ans.sort();
    for v in ans {
        println!("{v}");
    }
}
