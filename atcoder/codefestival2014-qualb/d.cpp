#include <algorithm>
#include <cassert>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <map>
#include <set>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
const double EPS=1e-9;

template<class I>
class SegTree {
	int n;
	vector<int> dat;
	I (*op) (const I&, const I&);
	I e;
public:
	SegTree(int n_, I (*op) (const I&, const I&), const I& e) : op(op), e(e) {
		n = 1;
		while (n < n_) { n *= 2; } // n is a power of 2
		dat.resize(2 * n);
		for (int i = 0; i < 2 * n - 1; i++) {
			dat[i] = e;
		}
	}
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

int n;
int a[100010];

int mymax(int x,int y) {return max(x,y);}

int main(void){
	cin>>n;
	REP(i,0,n) {
		cin >> a[i];
	}
	SegTree<int> st(n, max<int>, 0);
	st.update_array(0, n, a);
	REP(i,0,n) {
		ll s = 0;
		int l = i;
		int r = n;
		if (1) { // maximum x s.t. query(i + 1, x) <= a[i]   ( i <= x <= n - 1)
			while (r - l >= 2) {
				int m = (l + r) / 2;
				if (st.query(i+1, m) > a[i]) {
					r = m;
				} else {
					l = m;
				}
			}
			s = l - i;
		}
		l = -1, r = i;
		while (r - l >= 2) { // minimum x s.t. query(x, i - 1) <= a[i]  ( 0 <= x <= i)
			int m = (l + r) / 2;
			if (st.query(m, i - 1) > a[i]) {
				l = m;
			} else {
				r = m;
			}
		}
		s += i - r;
		cout << s << endl;
	}
	
}
