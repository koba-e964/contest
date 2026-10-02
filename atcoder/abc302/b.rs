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
    let get = |x: i32, y: i32| {
        if x as usize >= h || y as usize >= w {
            return '+';
        }
        s[x as usize][y as usize]
    };
    for i in 0..h {
        for j in 0..w {
            let ii = i as i32;
            let jj = j as i32;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if (dx, dy) == (0, 0) { continue; }
                    if (0..5).all(|k| get(ii + dx * k, jj + dy * k) == b"snuke"[k as usize] as char) {
                        for k in 0..5 {
                            println!("{} {}", ii + dx * k + 1, jj + dy * k + 1);
                        }
                        return;
                    }
                }
            }
        }
    }
}
