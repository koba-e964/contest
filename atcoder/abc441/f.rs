fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, m] = ints[..] else { panic!() };
    let mut pv = vec![];
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let [p, v] = ints[..] else { panic!() };
        pv.push((p, v as i64));
    }
    let mut lft = vec![vec![0; m + 1]; n + 1];
    let mut rgt = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        let (p, v) = pv[i];
        for j in 0..=m {
            lft[i + 1][j] = lft[i][j];
            if j >= p {
                lft[i + 1][j] = lft[i + 1][j].max(lft[i][j - p] + v);
            }
        }
    }
    for i in (0..n).rev() {
        let (p, v) = pv[i];
        for j in 0..=m {
            rgt[i][j] = rgt[i + 1][j];
            if j >= p {
                rgt[i][j] = rgt[i][j].max(rgt[i + 1][j - p] + v);
            }
        }
    }
    let opt = lft[n][m];
    for i in 0..n {
        let (p, v) = pv[i];
        let mut a = 0;
        let mut b = 0;
        for j in 0..=m {
            a = a.max(lft[i][j] + rgt[i + 1][m - j]);
        }
        for j in 0..=m - p {
            b = b.max(lft[i][j] + rgt[i + 1][m - p - j] + v);
        }
        print!("{}", match (a == opt, b == opt) {
            (true, true) => 'B',
            (false, true) => 'A',
            (true, false) => 'C',
            _ => panic!(),
        });
    }
}
