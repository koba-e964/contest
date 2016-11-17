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
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

/**
 * Segment Tree. This data structure is useful for fast folding on intervals of an array
 * whose elements are elements of monoid M. Note that constructing this tree requires the identity
 * element of M and the operation of M.
 * Header requirement: vector, algorithm
 * Verified by AtCoder ABC017-D (http://abc017.contest.atcoder.jp/submissions/660402)
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
  /*
    Updates all elements. O(n)
   */
  void update_all(const I *vals, int len) {
    for (int k = 0; k < std::min(n, len); ++k) {
      dat[k + n - 1] = vals[k];
    }
    for (int k = std::min(n, len); k < n; ++k) {
      dat[k + n - 1] = e;
    }
    for (int b = n / 2; b >= 1; b /= 2) {
      for (int k = 0; k < b; ++k) {
	dat[k + b - 1] = op(dat[k * 2 + b * 2 - 1], dat[k * 2 + b * 2]);
      }
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


std::vector<int> perm_inv(const std::vector<int> &p) {
  int len = p.size();
  std::vector<int> ans(len);
  for (int i = 0; i < len; ++i) {
    assert (0 <= p[i] && p[i] < len);
    ans[p[i]] = i;
  }
  return ans;
}

// q after p
std::vector<int> perm_comp(const std::vector<int> &q, const std::vector<int> &p) {
  int n = p.size();
  std::vector<int> ret(n);
  for (int i = 0; i < n; ++i) {
    ret[i] = q[p[i]];
  }
  return ret;
}

ll count_inversion(const VI &perm) {
  int n = perm.size();
  SegTree<int, plus<int> > st(n, plus<int>(), 0);
  // b's inversion number
  ll inv_c = 0;
  REP(i, 0, n) {
    inv_c += st.query(perm[i] + 1, n - 1);
    st.update(perm[i], st.query(perm[i], perm[i]) + 1);
  }
  return inv_c;
}

const int DEBUG = 0;

int main(void){
  int n;
  cin >> n;
  VI a(n), b(n);
  REP(i, 0, n) {
    cin >> a[i];
    a[i]--;
  }
  REP(i, 0, n) {
    cin >> b[i];
    b[i]--;
  }
  b = perm_comp(perm_inv(a), b);
  SegTree<int, plus<int> > st(n, plus<int>(), 0);
  // b's inversion number
  ll inv_c = count_inversion(b);
  if (inv_c % 2 == 1) {
    cout << -1 << endl;
    return 0;
  }
  st = SegTree<int, plus<int> >(n, plus<int>(), 0);
  if (DEBUG) {
    cerr << "inv_c = " << inv_c << endl;
  }
  ll half_c = inv_c / 2;

  VI front;
  
  REP(i, 0, n) {
    if (half_c <= 0) {
      break;
    }
    int cost = st.query(b[i] + 1, n - 1);
    if (half_c >= cost) {
      front.push_back(b[i]);
      half_c -= cost;
      st.update(b[i], st.query(b[i], b[i]) + 1);
    } else {
      break;
    }
  }
  sort(front.begin(), front.end());
  int pos = front.size();
  int inspos = -1;
  if (half_c >= 0) {
    int s = front.size();
    if (s < n) {
      inspos = pos - half_c;
      pos++;
    }
  }
  VI c(n);
  {
    int cur = 0;
    if (inspos == -1) {
      assert (front.size() == n);
      c = front;
    } else {
      REP(i, 0, inspos) {
	c[cur++] = front[i];
      }
      c[cur++] = b[pos - 1];
      REP(i, inspos, pos - 1) {
	c[cur++] = front[i];
      }
      REP(i, pos, n) {
	c[cur++] = b[i];
      }
    }
  }
  VI id(n);
  REP(i, 0, n) { id[i] = i; }
  if (DEBUG) {
    cout << "dist(id, c) = " << count_inversion(c) << endl;
    cout << "dist(c, b) = " << count_inversion(perm_comp(perm_inv(b), c)) << endl;
    cout << "b:";
    REP(i, 0, n) {
      cout << " " << b[i];
    }
    cout << endl << "c:";
    REP(i, 0, n) {
      cout << " " << c[i];
    }
    cout << endl << "b^-1c:";
    VI bcinv = perm_comp(perm_inv(b), c);
    REP(i, 0, n) {
      cout << " " << bcinv[i];
    }
    cout << endl;
  }
  c = perm_comp(a, c);
  b = perm_comp(a, b);
  if (DEBUG) {
    cout << "dist(a, c) = " << count_inversion(perm_comp(perm_inv(a), c)) << endl;
    cout << "dist(c, b) = " << count_inversion(perm_comp(perm_inv(b), c)) << endl;
  }
  REP(i, 0, n) {
    cout << c[i] + 1 << (i == n - 1 ? "\n" : " ");
  }
}
