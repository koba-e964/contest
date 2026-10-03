fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn calc(xtbl: &[[i64; 26]], ytbl: &[[i64; 26]], lens: &[i64], whole: &[[i64; 26]], n: usize, l: i64, c: usize) -> i64 {
    assert!(l <= lens[n]);
    assert!(l >= 0);
    if n == 1 {
        return xtbl[l as usize][c];
    }
    if n == 2 {
        return ytbl[l as usize][c];
    }
    if l == 0 {
        return 0;
    }
    if l == lens[n] {
        return whole[n][c];
    }
    if l > lens[n - 1] {
        let val = calc(xtbl, ytbl, lens, whole, n - 1, lens[n - 1], c);
        let val = val + calc(xtbl, ytbl, lens, whole, n - 2, l - lens[n - 1], c);
        return val;
    }
    let val = calc(xtbl, ytbl, lens, whole, n - 1, l, c);
    return val;
}

fn main() {
    let x = getline().trim().chars().collect::<Vec<_>>();
    let y = getline().trim().chars().collect::<Vec<_>>();
    let q = getline().trim().parse::<usize>().unwrap();
    fn make(x: &[char]) -> Vec<[i64; 26]> {
        let n = x.len();
        let mut ans = vec![[0; 26]; n + 1];
        for i in 0..n {
            ans[i + 1] = ans[i];
            ans[i + 1][(x[i] as u8 - b'a') as usize] += 1;
        }
        ans
    }
    let xtbl = make(&x);
    let ytbl = make(&y);
    let mut lens = vec![0; 3];
    let mut whole = vec![[0; 26]; 3];
    lens[1] = x.len() as i64;
    lens[2] = y.len() as i64;
    for i in 0.. 26 {
        whole[1][i] = xtbl[x.len()][i];
        whole[2][i] = ytbl[y.len()][i];
    }
    while lens[lens.len() - 1] < 1 << 60 {
        let i = lens.len();
        let val = lens[i - 1].saturating_add(lens[i - 2]);
        lens.push(val);
        let mut tmp = [0; 26];
        for j in 0..26 {
            tmp[j] = whole[i - 1][j] + whole[i - 2][j];
        }
        whole.push(tmp);
    }

    for _ in 0..q {
        let vals = getline().trim().split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let l = vals[0].parse::<i64>().unwrap() - 1;
        let r = vals[1].parse::<i64>().unwrap();
        let c = vals[2].chars().next().unwrap();
        let c = (c as u8 - b'a') as usize;
        let ans = calc(&xtbl, &ytbl, &lens, &whole, lens.len() - 1, r, c);
        let ans = ans - calc(&xtbl, &ytbl, &lens, &whole, lens.len() - 1, l, c);
        println!("{ans}");
    }
}
