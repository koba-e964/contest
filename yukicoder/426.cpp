#include <algorithm>
#include <cassert>
#include <iostream>
#include <list>
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

typedef vector<VL> VVL;
VVL add(const VVL &a, const VVL &b) {
  assert (a.size() == b.size());
  int n = a.size();
  int m = a[0].size();
  VVL ret(n, VL(m, 0));
  REP(i, 0, n) {
    REP(j, 0, m) {
      ret[i][j] = (a[i][j] + b[i][j]) % mod;
    }
  }
  return ret;
}
VVL mul(const VVL &a, const VVL &b) {
  assert (a[0].size() == b.size());
  int n = a.size();
  int m = b.size();
  int l = b[0].size();
  VVL ret(n, VL(l, 0));
  REP(i, 0, n) {
    REP(j, 0, m) {
      REP(k, 0, l) {
	ret[i][k] += a[i][j] * b[j][k];
	ret[i][k] %= mod;
      }
    }
  }
  return ret;
}

struct dat {
  VVL a, b, s;
};

dat elem_mul(const dat &abs1, const dat &abs2) {
  VVL a = mul(abs2.a, abs1.a);
  VVL b = mul(abs1.b, abs2.b);
  VVL s = add(abs1.s, mul(mul(abs1.b, abs2.s), abs1.a));
  return dat{a, b, s};
}

VVL mat_s(int i) {
  VVL s(2, VL(3, 0));
  s[0][0] = 6 * i;
  s[0][1] = 6 * i + 1;
  s[0][2] = 6 * i + 2;
  s[1][0] = 6 * i + 3;
  s[1][1] = 6 * i + 4;
  s[1][2] = 6 * i + 5;
  return s;
}

int main(void){
  ios::sync_with_stdio(false);
  int n, q;
  cin >> n;
  VVL a(3, VL(1)), b(2, VL(1));
  REP(i, 0, 3) {
    cin >> a[i][0];
  }
  REP(i, 0, 2) {
    cin >> b[i][0];
  }
  VVL unit3(3), unit2(2), dummy23(2, VL(3));
  REP(i, 0, 3) {
    VL t(3);
    t[i] = 1;
    unit3[i] = t;
  }
  REP(i, 0, 2) {
    VL t(2);
    t[i] = 1;
    unit2[i] = t;
  }
  
  SegTree<dat, dat (*) (const dat &, const dat &) > st(n + 1, elem_mul, dat{unit3, unit2, dummy23});
  {
    vector<dat> ary(n + 1);
    REP(i, 0, n + 1) {
      ary[i] = dat{unit3, unit2, mat_s(i)};
    }
    st.update_all(&ary[0], n + 1);
  }
  cin >> q;
  REP(loop_cnt, 0, q) {
    string qty;
    int i;
    cin >> qty;
    if (qty == "a") {
      cin >> i;
      REP(k, 0, 3) {
	REP(j, 0, 3) {
	  cin >> unit3[k][j];
	}
      }
      dat cur = st.query(i, i);
      st.update(i, dat{unit3, cur.b, mat_s(i)});
    }
    if (qty == "b") {
      cin >> i;
      REP(k, 0, 2) {
	REP(j, 0, 2) {
	  cin >> unit2[k][j];
	}
      }
      dat cur = st.query(i, i);
      st.update(i, dat{cur.a, unit2, mat_s(i)});
    }
    if (qty == "ga") {
      cin >> i;
      VVL res = mul(st.query(0, i - 1).a, a);
      cout << res[0][0] << " " << res[1][0] << " " << res[2][0] << "\n";
    }
    if (qty == "gb") {
      cin >> i;
      dat tmp = st.query(i + 1, n);
      VVL res = add(mul(tmp.b, b),
		    mul(tmp.s, mul(st.query(0, i).a, a)));
      cout << res[0][0] << " " << res[1][0] << "\n";
    }
  }
}
