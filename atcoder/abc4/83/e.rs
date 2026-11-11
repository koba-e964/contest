fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

#[allow(unused)]
trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, m] = ints[..] else { panic!() };
    let mut same = vec![];
    let mut typo = vec![];
    let mut zero = 0i64;
    let mut one = 0i64;
    let mut two = 0i64;
    for _ in 0..n {
        let s = getline().trim().chars().collect::<Vec<_>>();
        let mut x = 0;
        for i in 0..m {
            x *= 32;
            x += (s[i] as u8 - b'a') as u32;
        }
        same.push(x);
        for i in 0..m {
            let me = (x >> (5 * i)) & 31;
            for j in 0..26 {
                if me == j { continue; }
                let y = x & !(31 << (5 * i));
                let y = y | (j << (5 * i));
                typo.push(y);
            }
        }
    }
    same.sort_unstable();
    typo.sort_unstable();
    let mut cur = !0u32;
    let mut cnt = 0i64;
    for &s in &same {
        if cur == s {
            cnt += 1;
        } else {
            cnt = 0;
        }
        cur = s;
        zero += cnt;
        two -= cnt * 25 * m as i64;
    }
    cur = !0u32;
    cnt = 0;
    for &y in &typo {
        let lo = same.lower_bound(&y);
        let hi = same.upper_bound(&y);
        one += (hi - lo) as i64;
        two -= (hi - lo) as i64 * 12;
        if cur == y {
            cnt += 1;
        } else {
            cnt = 0;
        }
        cur = y;
        two += cnt;
    }
    eprintln!("zero = {zero}, one = {one}, two = {two}");
    println!("{}", zero + one / 2 + two / 2);
}
