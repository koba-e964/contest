// ax^2 + bx + c = 0
// None: infinitely many solutions
fn quadratic(a: i64, b: i64, c: i64) -> Option<Vec<f64>> {
    if a == 0 {
        if b == 0 {
            if c == 0 {
                return None;
            } else {
                return Some(vec![]);
            }
        }
        return Some(vec![-c as f64 / b as f64]);
    }
    let mut ans = vec![];
    if b * b / 4 >= a * c {
        let cent = -b as f64 / 2.0 / a as f64;
        let delta = ((b as f64 * b as f64 - 4.0 * a as f64 * c as f64) as f64).sqrt() / 2.0 / a.abs() as f64;
        ans.push(cent - delta);
        if b * b % 4 != 0 || b * b / 4 > a * c {
            ans.push(cent + delta);
            if ans[0].abs() < ans[1].abs() {
                ans[0] = c as f64 / a as f64 / ans[1];
            } else {
                ans[1] = c as f64 / a as f64 / ans[0];
            }
        }
    }
    Some(ans)
}
