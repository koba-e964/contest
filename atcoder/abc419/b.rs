fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let q: i32 = getline().trim().parse().unwrap();
    let mut t = 0;
    let mut s = std::collections::BTreeSet::new();
    for _ in 0..q {
        let ints = getline()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if ints[0] == 1 {
            let x = ints[1];
            s.insert((x, t));
            t += 1;
        } else {
            let &y = s.iter().next().unwrap();
            s.remove(&y);
            println!("{}", y.0);
        }
    }
}
