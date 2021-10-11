// Depends on: tree/Reroot.rs
// Verified by: https://yukicoder.me/submissions/706820
impl LeaveOne for MexLeaveOne {
    type T = usize;
    type App = ();
    fn build(vals: &[usize], _: &()) -> Self {
        let seen = vals.iter().cloned().collect::<std::collections::HashSet<_>>();
        let mut mex1 = 0;
        while seen.contains(&mex1) {
            mex1 += 1;
        }
        let mut mex2 = mex1 + 1;
        while seen.contains(&mex2) {
            mex2 += 1;
        }
        let mut f = vec![0; mex2];
        for &v in vals {
            if v < mex2 {
                f[v] += 1;
            }
        }
        let loo = MexLeaveOne {
            mex1: mex1,
            mex2: mex2,
            f: f,
        };
        loo
    }
    fn leave_one(&self, excl: usize) -> usize {
        if excl >= self.mex1 {
            return self.mex1;
        }
        if self.f[excl] == 1 {
            return excl;
        }
        self.mex1
    }
    fn exchange_one(&self, excl: usize, incl: usize) -> usize {
        if excl == incl {
            return self.mex1;
        }
        let mex = self.add_one(incl);
        if excl >= mex {
            return mex;
        }
        let res = if self.f[excl] == 1 {
            excl
        } else {
            mex
        };
        res
    }
    fn add_one(&self, incl: usize) -> usize {
        if incl == self.mex1 {
            self.mex2
        } else {
            self.mex1
        }
    }
    fn as_is(&self) -> usize {
        self.mex1
    }
}
