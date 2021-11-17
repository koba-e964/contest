// Depends on: tree/Reroot.rs
// Verified by: https://yukicoder.me/submissions/717057
#[derive(Clone, Default)]
// (#edges used, do we have an objective?)
struct UsedLeaveOne(i64, i32);

impl LeaveOne for UsedLeaveOne {
    type T = (i64, bool);
    type Me = bool;
    type App = ();
    fn build(me: bool, vals: &[(i64, bool)], _: &()) -> Self {
        let mut res = 0;
        let mut me = if me { 1 } else { 0 };
        for &(v, has) in vals {
            if has {
                res += v + 1;
                me += 1;
            }
        }
        UsedLeaveOne(res, me)
    }
    fn leave_one(&self, excl: (i64, bool)) -> (i64, bool) {
        let UsedLeaveOne(mut res, mut me) = *self;
        if excl.1 {
            res -= excl.0 + 1;
            me -= 1;
        }
        if me > 0 {
            (res, true)
        } else {
            (0, false)
        }
    }
    fn exchange_one(&self, excl: (i64, bool), incl: (i64, bool)) -> (i64, bool) {
        let (res, me) = self.leave_one(excl);
        (res + if incl.1 { incl.0 + 1 } else { 0 }, incl.1 || me)
    }
    fn add_one(&self, incl: (i64, bool)) -> (i64, bool) {
        (self.0 + if incl.1 { incl.0 + 1 } else { 0 }, incl.1 || self.1 > 0)
    }
    fn as_is(&self) -> (i64, bool) {
        (self.0, self.1 > 0)
    }
}
