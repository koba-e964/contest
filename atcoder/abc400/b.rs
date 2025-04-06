fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let line = getline();
    let tokens = line.trim().split(' ').collect::<Vec<_>>();
    let n = tokens[0].parse::<u64>().unwrap();
    let m = tokens[1].parse::<u64>().unwrap();
    let mut cur = 1u64;
    let mut sum = 0u64;
    for _ in 0..m + 1 {
        sum = sum.saturating_add(cur);
        cur = cur.saturating_mul(n);
    }
    if sum > 1_000_000_000 {
        println!("inf");
    } else {
        println!("{sum}");
    }
}
