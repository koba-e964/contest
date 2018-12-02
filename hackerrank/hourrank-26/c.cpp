#include <iostream>
#include <string>
#include <algorithm>
#include <utility>
#include <set>
#include <cassert>
#include <vector>

using namespace std;

#define REP(i, s, n) for (int i = (int)(s); i < (int)(n); ++i)

typedef long long ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
typedef pair<int,int> pii;

// http://d.hatena.ne.jp/sune2/20140310/1394440369
const ll INF = 1LL<<61;
struct CHT2 {  
  CHT2() {
    S.insert(L(INF,0));
    S.insert(L(-INF,0));
    C.insert(cp(L(INF,0),L(-INF,0)));
  }
  // for debug
  void print() {
    cout << "S : "; for (auto it : S) printf("(%lld,%lld)", it.a, it.b); puts("");
    cout << "C : "; for (auto it : C) printf("(%lld,%lld)", it.n, it.d); puts("");
  }
  // |ab| < LLONG_MAX/4 ???
  void add(ll a, ll b) {
    const L p(a,b);
    It pos = S.insert(p).first;
    if (check(*it_m1(pos), p, *it_p1(pos))) {
      S.erase(pos);
      return;
    }
    C.erase(cp(*it_m1(pos), *it_p1(pos)));
    {
      It it = it_m1(pos);
      while(it!=S.begin() && check(*it_m1(it), *it, p)) --it;
      C_erase(it, it_m1(pos));
      S.erase(++it,pos);
      pos = S.find(p);
    }
    {
      It it = it_p1(pos);
      while(it_p1(it)!=S.end() && check(p,*it, *it_p1(it))) ++it;
      C_erase(++pos, it);
      S.erase(pos, it);
      pos = S.find(p);
    }
    C.insert(cp(*it_m1(pos), *pos));
    C.insert(cp(*pos, *it_p1(pos)));
  }
  ll query(ll x) {
    const L &p = (--C.lower_bound(CP(x,1,L(0,0))))->p;
    return p.a*x + p.b;
  }
  
private:
  
  template<class T> T it_p1(T a) { return ++a; }
  template<class T> T it_m1(T a) { return --a; }  
  struct L {
    ll a, b;
    L(ll a, ll b) : a(a),b(b) {}
    bool operator<(const L &rhs) const {
      return a != rhs.a ? a > rhs.a : b < rhs.b;
    }
  };
  struct CP {
    ll n,d;
    L p;
    CP(ll _n, ll _d, const L &p) : n(_n),d(_d),p(p) {
      if (d < 0) { n *= -1; d *= -1; }
    };
    bool operator<(const CP &rhs) const {
      if (n == INF || rhs.n == -INF) return 0;
      if (n == -INF || rhs.n == INF) return 1;      
      return n * rhs.d < rhs.n * d;
    }
  };
  set<L> S;
  set<CP> C;

  typedef set<L>::iterator It;
  
  void C_erase(It a, It b) {
    for (It it=a; it!=b; ++it)
      C.erase(cp(*it, *it_p1(it)));
  }
  CP cp(const L &p1, const L &p2) {
    if (p1.a == INF) return CP(-INF,1,p2);
    if (p2.a == -INF) return CP(INF,1,p2);
    return CP(p1.b-p2.b, p2.a-p1.a, p2);
  }
  bool check(const L &p1, const L &p2, const L &p3) {
    if (p1.a==p2.a && p1.b <= p2.b) return 1;
    if (p1.a == INF || p3.a == -INF) return 0;
    return (p2.a-p1.a)*(p3.b-p2.b) >= (p2.b-p1.b)*(p3.a-p2.a);
  }
};


int main(void) {
  ios_base::sync_with_stdio(false);
  cin.tie(0);
  CHT2 cht;
  int n;
  cin >> n;
  VL a(n);
  REP(i, 0, n) cin >> a[i];
  VL acc(n + 1), acc2(n + 1);
  REP(i, 0, n) {
    acc[i + 1] = acc[i] + a[i];
    acc2[i + 1] = acc2[i] + a[i] * a[i];
  }
  ll ma = -1e18;
  REP(i, 0, n) {
    ll val = -cht.query(acc[i]) + acc[i] * acc[i] - acc2[i];
    ma = max(ma, val);
    ll a = -2 * acc[i];
    ll b = acc[i] * acc[i] + acc2[i];
    cht.add(-a, -b); // Note: reversed because we want maximum values.
  }
  cout << ma / 2 << "\n";
}
