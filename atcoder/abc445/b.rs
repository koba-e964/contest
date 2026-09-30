fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    let mut s = vec![];
    let mut ma = 0;
    for _ in 0..n {
        let t = getline().trim().to_string();
        ma = ma.max(t.len());
        s.push(t);
    }
    for s in s {
        let k = (ma - s.len()) / 2;
        let tmp = str::repeat(".", k);
        println!("{}", tmp.clone() + &s + &tmp);
    }
}
