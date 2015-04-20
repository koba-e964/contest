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

const int N = 10010;
int n;

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

int c;
double l[N];

struct D {
  double x, y;
  double theta;
  D(double x, double y, double theta) : x(x), y(y), theta(theta) {}
  D() : x(0), y(0), theta(0) {}
};

struct adder {
  D operator ()(const D& a, const D& b) {
    double t = a.theta;
    double x3, y3;
    x3 = b.x * cos(t) - b.y * sin(t);
    y3 = b.x * sin(t) + b.y * cos(t);
    x3 += a.x;
    y3 += a.y;
    return D(x3, y3, a.theta + b.theta);
  }
};


const double pi = acos(-1);

int main(void) {
  while (1) {
    int qq = scanf("%d %d", &n, &c);
    if (qq != 2) {
      break;
    }
    SegTree<D, adder> st(n + 1, adder(), D());
    rep(i, 0, n) {
      scanf("%lf", &l[i]);
      st.update(i, D(l[i], 0, 0));
    }
    rep(i, 0, c) {
      int s;
      double a;
      scanf("%d %lf", &s, &a);
      s--;
      a -= 180;
      D q = D(l[s], 0, a / 180.0 * pi);
      st.update(s, q);
      D tot = st.query(0, n - 1);
      printf("%.2f %.2f\n", -tot.y, tot.x);
    }
    printf("\n");
  }
  return 0;
}
