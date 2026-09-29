fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let q = getline().trim().parse::<i32>().unwrap();
    let s = getline().trim().chars().collect::<Vec<_>>();
    let t = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut acc = vec![0; n + 1];
    for i in t.len()..n + 1 {
        let j = i - t.len();
        acc[j + 1] = acc[j] + if s[j..i] == t {
            1
        } else {
            0
        };
    }
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let [l, r] = ints[..] else { panic!() };
        let l = l - 1;
        if r - l < t.len() {
            println!("No");
        } else {
            println!("{}", if acc[r + 1 - t.len()] > acc[l] { "Yes" } else { "No" });
        }
    }
}
