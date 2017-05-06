import java.io.*;
import java.util.*;
import java.util.function.*;

/*
 * Sparse Table.
 * BiOp should be the type of a binary operator which is
 * associative, commutative and idempotent.
 * (For example, min and gcd satisfy them.)
 * Header Requirement: vector, cassert
 * Verified by: AtCoder ARC023 D (http://arc023.contest.atcoder.jp/submissions/960757)
 */
class SparseTable<T> {
    private BinaryOperator<T> biop;
    Object[][] st;
    void create_sparse_table(int n, T[] lcp) {
	int h = 1;
	while ((1 << h) < n) {
	    ++h;
	}
	st = new Object[h + 1][n];

	for (int i = 0; i < n; ++i) {
	    st[0][i] = lcp[i];
	}
	for (int j = 1; j <= h; ++j) {
	    for (int i = 0; i <= n - (1 << j); ++i) {
		st[j][i] = biop.apply((T)st[j - 1][i], (T)st[j - 1][i + (1 << (j-1))]);
	    }
	}
    }
    static int top_bit(int t) {
	return 31 - Integer.numberOfLeadingZeros(t);
    }
    /*
     * Initializes this sparse table. O(n log n) where n = ary.size().
     */
    public SparseTable(BinaryOperator<T> biop, T[] ary) {
	this.biop = biop;
	create_sparse_table(ary.length, ary);
    }
    /*
     * Computes biop(ary[f], ary[f+1], ..., ary[s]). O(1).
     * Note: the interval is inclusive.
     */
    public T query(int f, int s) {
	assert (f <= s);
	int diff = top_bit(s + 1 - f);
	return biop.apply((T) st[diff][f], (T) st[diff][s + 1 - (1 << diff)]);
    }
};



class Main {
    public static void main(String[] args) {
	Scanner sc = new Scanner(System.in);
	int n = sc.nextInt();
	assert (n <= 100000);
	String[] s = new String[n];
	int tot_len = 0;
        for (int i = 0; i < n; ++i) {
	    s[i] = sc.next();
	    tot_len += s[i].length();
	}
	assert (tot_len <= 800000);

	Integer[] perm = new Integer[n];
	int[] inv_perm = new int[n];
	for (int i = 0; i < n; ++i) { perm[i] = i; }
	Arrays.sort(perm, (Integer x, Integer y) -> s[x].compareTo(s[y]));
        for (int i = 0; i < n; ++i) { inv_perm[perm[i]] = i; }
  
	int m = sc.nextInt();
	assert (m <= 3000000);
	long x, d;
	x = sc.nextLong();
	d = sc.nextLong();
	long lim = (long) n * (long)(n - 1);
	assert (0 <= x);
	assert (x < lim);
	assert (d < lim);
	assert (0 <= d);
	
        Integer[] lcp = new Integer[n - 1];
	for (int i = 0; i < n - 1; ++i) {
	    int pos = 0;
	    String t1 = s[perm[i]];
	    String t2 = s[perm[i + 1]];
	    int lim1 = Math.min(t1.length(), t2.length());
	    for (; pos < lim1; ++pos) {
		if (t1.charAt(pos) != t2.charAt(pos)) { break; }
	    }
	    lcp[i] = pos;
	}
	
	SparseTable<Integer> st = new SparseTable<>((Integer xx, Integer yx) -> Integer.min(xx, yx), lcp);
	long total = 0;
	for (int loop_cnt = 0; loop_cnt < m; ++loop_cnt) {
	    int i, j;
	    i = (int) ((x / (n - 1)) + 1);
	    j = (int) ((x % (n - 1)) + 1);
	    if (i > j) {
		int tmp = i;
		i = j;
		j = tmp;
	    } else {
		j++;
	    }
	    assert (1 <= i);
	    assert (i < j);
	    assert (j <= n);
	    i--; j--;
	    int ii = inv_perm[i];
	    int ij = inv_perm[j];
	    if (ii > ij) {
		int tmp = ii;
		ii = ij;
		ij = tmp;
	    }
	    assert (ii < ij);
	    total += st.query(ii, ij - 1);
	    x = (x + d) % ((long)n  * (long)(n - 1));
	}
	System.out.println(total);
    }
}
