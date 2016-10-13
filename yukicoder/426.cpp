#include <algorithm>
#include <cassert>
#include <iostream>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;
const ll mod = 1e9 + 7;

typedef vector<VL> VVL;
VVL add(const VVL &a, const VVL &b) {
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
struct dat{VVL a, b, s;};
dat elem_mul(const dat &abs1, const dat &abs2) {
  VVL a = mul(abs2.a, abs1.a);
  VVL b = mul(abs1.b, abs2.b);
  VVL s = add(abs1.s, mul(mul(abs1.b, abs2.s), abs1.a));
  return dat{a, b, s};
}

const int N = 1 << 17;
dat ary[2*N];
dat st_e;
void init(dat e){
  for (int i = 0; i < 2 * N - 1; i++) {
    ary[i] = e;
  }
  st_e = e;
}
void update(int k, dat v) {
  k += N - 1;
  ary[k] = v;
  while (k > 0) {
    k = (k - 1) / 2;
    ary[k] = elem_mul(ary[2 * k + 1], ary[2 * k + 2]);
  }
}
void update_all(const dat *vals, int len) {
  for (int k = 0; k < std::min(N, len); ++k) {
    ary[k + N - 1] = vals[k];
  }
  for (int k = std::min(N, len); k < N; ++k) {
    ary[k + N - 1] = st_e;
  }
  for (int b = N / 2; b >= 1; b /= 2) {
    for (int k = 0; k < b; ++k) {
      ary[k + b - 1] = elem_mul(ary[k * 2 + b * 2 - 1], ary[k * 2 + b * 2]);
    }
  }
}
dat querySub(int a, int b, int k, int l, int r) {
  if (r <= a || b <= l) return st_e;
  if (a <= l && r <= b) return ary[k];
  dat vl = querySub(a, b, 2 * k + 1, l, (l + r) / 2);
  dat vr = querySub(a, b, 2 * k + 2, (l + r) / 2, r);
  return elem_mul(vl, vr);
}
dat query(int a, int b) {
  return querySub(a, b + 1, 0, 0, N);
}
int main(void){
  int n, q;
  cin >> n;
  VVL a(3, {0}), b(2, {0});
  REP(i, 0, 3) {
    cin >> a[i][0];
  }
  REP(i, 0, 2) {
    cin >> b[i][0];
  }
  VVL u3(3), u2(2);
  REP(i, 0, 3) {
    VL t(3);
    t[i] = 1;
    u3[i] = t;
  }
  REP(i, 0, 2) {
    VL t(2);
    t[i] = 1;
    u2[i] = t;
  }
  
  init({u3, u2, VVL(2, VL(3))});
  vector<dat> ary(n + 1);
  REP(i, 0, n + 1) {
    VVL s(2, VL(3, 0));
    REP(k,0,6)s[k/3][k%3]=6*i+k;
    ary[i] = {u3, u2, s};
  }
  update_all(&ary[0], n + 1);
  cin >> q;
  REP(z, 0, q) {
    string qty;
    int i;
    cin >> qty >> i;
    dat cur = query(i, i);
    if (qty == "a") {
      REP(k, 0, 3) {
	REP(j, 0, 3) {
	  cin >> cur.a[k][j];
	}
      }
      update(i, cur);
    }
    if (qty == "b") {
      REP(k, 0, 2) {
	REP(j, 0, 2) {
	  cin >> cur.b[k][j];
	}
      }
      update(i, cur);
    }
    if (qty == "ga") {
      VVL r = mul(query(0, i - 1).a, a);
      cout << r[0][0] << " " << r[1][0] << " " << r[2][0] << "\n";
    }
    if (qty == "gb") {
      dat tmp = query(i + 1, n);
      VVL r = add(mul(tmp.b, b), mul(tmp.s, mul(query(0, i).a, a)));
      cout << r[0][0] << " " << r[1][0] << "\n";
    }
  }
}
