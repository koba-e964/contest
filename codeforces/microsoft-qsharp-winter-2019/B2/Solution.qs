namespace Solution {
    open Microsoft.Quantum.Primitive;
    open Microsoft.Quantum.Canon;
    open Microsoft.Quantum.Math;
    open Microsoft.Quantum.Arithmetic;
    open Microsoft.Quantum.Preparation;
    open Microsoft.Quantum.Convert;

    // The author read the tutorial.
    operation Solve (q : Qubit) : Int {
        mutable ans = -1;
        using (anc = Qubit()) {
            // 1 tensor Z
            Z(q);
            // diag(1, H, 1)
            centH([q, anc]);
            // S tensor 1
            S(anc);
            // R (controlled on 0)
            let angle = ArcCos(1.0 / Sqrt(3.0));
            (ControlledOnInt(0, Ry(2.0 * angle, _)))([anc], q);
            // diag(1, H, 1)
            centH([q, anc]);

            set ans = MeasureInteger(LittleEndian([q, anc]));
            Reset(anc);
        }
        return ans;
    }

    operation centH(qs: Qubit[]): Unit is Adj + Ctl {
        CNOT(qs[0], qs[1]);
        Controlled H([qs[1]], qs[0]);
        CNOT(qs[0], qs[1]);
    }

    operation Test(count: Int, kind: Int): Int[] {
        mutable freq = new Int[4];
        let fac = 1.0 / Sqrt(2.0);
        let coef = [
            ComplexPolar(fac, 0.0),
            ComplexPolar(fac, PI() * 2.0 / 3.0 * IntAsDouble(kind))
        ];
        for (_ in 0..count - 1) {
            using (q = Qubit()) {
                PrepareArbitraryState(coef, LittleEndian([q]));
                let ans = Solve(q);
                set freq = freq w/ ans <- freq[ans] + 1;
                Reset(q);
            }
        }
        return freq;
    }
}
