#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <stack>
#include <bitset>
#include <algorithm>
#include <functional>
#include <numeric>
#include <utility>
#include <sstream>
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <cmath>
#include <cstdlib>
#include <cctype>
#include <string>
#include <cstring>
#include <ctime>

using namespace std;

typedef long long ll;
typedef vector<int> VI;
typedef pair<int, int> PI;
const double EPS = 1e-9;
#define rep(i,s,n) for(int i=(s); i < int(n); ++i)

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

const int N = 1e6;
int si[N], sj[N];
int n, m;
const int inf = 1e9;

struct minobj {
  int operator()(int x, int y) { return min(x, y);}
};

int main(void) {
  cin >> n >> m;
  rep(i, 0, m) {
    scanf("%d %d", si + i, sj + i);
    si[i] --;
    sj[i] --;
  }
  SegTree<int, minobj> st(n, minobj(), inf);
  st.update(0, 0);
  rep(i, 1, n) {
    st.update(i, inf);
  }
  rep(i, 0, m) {
    int m = st.query(si[i], sj[i]);
    int orig = st.query(sj[i], sj[i]);
    st.update(sj[i], min(m + 1, orig));
  }
  cout << st.query(n - 1, n - 1) << endl;
}

