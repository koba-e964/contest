#include <algorithm>
#include <cassert>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <string>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<ll> VL;
typedef pair<ll, ll> PL;

const ll inf = 1e14;

int main(void) {
  ios::sync_with_stdio(false);
  cin.tie(0);
  string a, b;
  cin >> a >> b;
  int n = a.size();
  VL lft(n, inf), rgt(n, inf);
  REP(i, 0, n) {
    ll r = i, l = (n - i) % n;
    REP(j, 0, n) {
      if (b[(i + j) % n] == '1') {
        lft[j] = min(lft[j], l);
        rgt[j] = min(rgt[j], r);
      }
    }
  }
  ll mi = inf;
  // <==---0=====>
  REP(stop, -n + 1, n) {
    vector<PL> pool;
    REP(i, 0, n) {
      if (b[(i + stop + n) % n] != a[i]) {
        pool.push_back(PL(lft[i], rgt[i]));
      }
    }
    sort(pool.begin(), pool.end());
    int m = pool.size();
    VL rma(m + 1, 0);
    for (int i = m - 1; i >= 0; --i) {
      rma[i] = max(rma[i + 1], pool[i].second);
    }
    REP(l, max(0, -stop), n) {
      int idx = upper_bound(pool.begin(), pool.end(), PL(l, inf + 1))
        - pool.begin();
      ll rmax = rma[idx];
      rmax = max(rmax, (ll)stop);
      mi = min(mi, 2 * (l + rmax) - abs(stop) + m);
    }
  }
  cout << (mi >= inf ? -1 : mi) << endl;
}
