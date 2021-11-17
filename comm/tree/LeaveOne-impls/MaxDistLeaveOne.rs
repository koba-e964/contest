// Depends on: tree/Reroot.rs
// Verified by: https://yukicoder.me/submissions/717057
#[derive(Clone, Default)]
struct MaxDistLeaveOne([i64; 2]);

impl LeaveOne for MaxDistLeaveOne {
    type T = i64;
    type Me = bool;
    type App = ();
    fn build(me: bool, vals: &[i64], _: &()) -> Self {
        const INF: i64 = 1 << 50;
        let mut res = [-INF; 2];
        if me {
            res = [0; 2];
        }
        for &v in vals {
            res[0] = std::cmp::max(res[0], v + 1);
            res.sort();
        }
        MaxDistLeaveOne(res)
    }
    fn leave_one(&self, excl: i64) -> i64 {
        if self.0[1] == excl + 1 {
            self.0[0]
        } else {
            self.0[1]
        }
    }
    fn exchange_one(&self, excl: i64, incl: i64) -> i64 {
        std::cmp::max(self.leave_one(excl), incl + 1)
    }
    fn add_one(&self, incl: i64) -> i64 {
        std::cmp::max(self.0[1], incl + 1)
    }
    fn as_is(&self) -> i64 {
        self.0[1]
    }
}
