namespace Solution {
    open Microsoft.Quantum.Intrinsic;

    operation Solve (unitary : (Qubit => Unit is Adj+Ctl)) : Int {
        mutable ans = -1;
        using (q0 = Qubit()) {
            H(q0);
            unitary(q0);
            unitary(q0);
            H(q0);
            if (M(q0) == One) {
                set ans = 1;
                X(q0);
            } else {
                set ans = 0;
            }
        }
        return ans;
    }
    operation TestI(count: Int): Int {
        mutable agree = 0;
        for (test in 1..count) {
            let res = Solve(I);
            if (res == 0) {
                set agree += 1;
            }
        }
        return agree;
    }
    operation TestS(count: Int): Int {
        mutable agree = 0;
        for (test in 1..count) {
            let res = Solve(S);
            if (res == 1) {
                set agree += 1;
            }
        }
        return agree;
    }
}
