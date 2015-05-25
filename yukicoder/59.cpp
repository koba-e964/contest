#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <numeric>
#include <queue>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS=1e-9;

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 */
template<class I, class BiOp = I (*) (I, I)>
class SegTree {
	int n;
	vector<I> dat;
	BiOp op;
	I e;
public:
	SegTree(int n_, BiOp op, I e) : op(op), e(e) {
		n = 1;
		while (n < n_) { n *= 2; } // n is a power of 2
		dat.resize(2 * n);
		for (int i = 0; i < 2 * n - 1; i++) {
			dat[i] = e;
		}
	}
	/* ary[k] <- v */
	void update(int k, I v) {
		k += n - 1;
		dat[k] = v;
		while (k > 0) {
			k = (k - 1) / 2;
			dat[k] = op(dat[2 * k + 1], dat[2 * k + 2]);
		}
	}
	void update_array(int k, int len, I *vals) {
		for (int i = 0; i < len; ++i) {
			update(k + i, vals[i]);
		}
	}
	/* l,r are for simplicity */
	I querySub(int a, int b, int k, int l, int r) {
		// [a,b) and  [l,r) intersects?
		if (r <= a || b <= l) return e;
		if (a <= l && r <= b) return dat[k];
		I vl = querySub(a, b, 2 * k + 1, l, (l + r) / 2);
		I vr = querySub(a, b, 2 * k + 2, (l + r) / 2, r);
		return op(vl, vr);
	}
	/* [a, b] (note: inclusive) */
	I query(int a, int b) {
		return querySub(a, b + 1, 0, 0, n);
	}
};

int main(void){
  const int M = 1048500;
  int n, k;
  cin >> n >> k;
  SegTree<int, plus<int> > st(M, plus<int>(), 0);
  REP (loop_var, 0, n) {
    int w; cin >> w;
    if (w > 0) {
      int q = st.query(w, M);
      if (q < k) {
	st.update(w, st.query(w, w) + 1);
      }
    } else {
      w = -w;
      int v = st.query(w, w);
      if (v > 0) {
	st.update(w, v - 1);
      }
    }
  }
  cout << st.query(1, M) << endl;
}
