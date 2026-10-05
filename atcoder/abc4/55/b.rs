fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [h, w] = ints[..] else { panic!() };
    let mut s = vec![];
    for _ in 0..h {
        let t = getline().trim().chars().collect::<Vec<_>>();
        s.push(t);
    }
    let mut ans = 0;
    for i in 0..h {
        for j in i + 1..h + 1 {
            for a in 0..w {
                for b in a + 1..w + 1 {
                    let mut ok = true;
                    for x in i..j {
                        for y in a..b {
                            if s[x][y] != s[i + j - 1 - x][a + b - 1 - y] {
                                ok = false;
                            }
                        }
                    }
                    if ok {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{ans}");
}
