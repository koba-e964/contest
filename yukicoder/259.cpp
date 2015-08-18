#include <vector>
/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 * Header requirement: vector
 */
template<class I, class BiOp = I (*) (I, I)>
class SegTree {
  int n;
  std::vector<I> dat;
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
  void update_array(int k, int len, const I *vals) {
    for (int i = 0; i < len; ++i) {
      update(k + i, vals[i]);
    }
  }
  /* l,r are for simplicity */
  I querySub(int a, int b, int k, int l, int r) const {
    // [a,b) and  [l,r) intersects?
    if (r <= a || b <= l) return e;
    if (a <= l && r <= b) return dat[k];
    I vl = querySub(a, b, 2 * k + 1, l, (l + r) / 2);
    I vr = querySub(a, b, 2 * k + 2, (l + r) / 2, r);
    return op(vl, vr);
  }
  /* [a, b] (note: inclusive) */
  I query(int a, int b) const {
    return querySub(a, b + 1, 0, 0, n);
  }
};

#include <iostream>
#include <algorithm>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;

ll tt(const SegTree<ll, plus<ll> > &st, int x, int y, int n) {
  if (x / n == y / n) {
    return st.query(x % n, y % n);
  }
  return st.query(x % n, n - 1) + st.query(0, y % n);
}

int main(void){
  int n, q;
  cin >> n >> q;
  SegTree<ll, plus<ll> > st(2 * n, plus<ll>(), 0);
  REP(i, 0, q) {
    string x;
    ll y, z;
    ll t;
    cin >> x >> t >> y >> z;
    int u = t % (2 * n);
    switch(x[0]) {
    case 'L':
      {
	int pos = (y + t) % (2 * n);
	st.update(pos, st.query(pos, pos) + z);
	break;
      }
    case 'R':
      {
	int pos = (-y + t + 2 * n - 1) % (2 * n);
	st.update(pos, st.query(pos, pos) + z);
	break;
      }
    case 'C':
      {
	ll q = tt(st, y + t, z + t - 1, 2 * n);
	q += tt(st, - z + t + 2 * n, -y + t + 2 * n - 1, 2 * n);
	cout << q << endl;
      }
    }
  }
}
