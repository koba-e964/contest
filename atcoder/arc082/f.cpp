#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
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



int main(void) {
  ll x;
  int k;
  cin >> x >> k;
  VL r(k + 1, 0);
  REP(i, 0, k) {
    cin >> r[i + 1];
  }
  VL el(k + 1), em(k + 1), en(k + 1);
  VL curdir(k + 1);
  el[0] = 0;
  em[0] = x;
  en[0] = 0; // max(0, min(x, a + 0))
  ll dir = -1;
  curdir[0] = -1;
  REP(i, 0, k) {
    ll pass = r[i + 1] - r[i];
    en[i + 1] = en[i] + dir * pass;
    el[i + 1] = max(0LL, min(x, el[i] + dir * pass));
    em[i + 1] = max(0LL, min(x, em[i] + dir * pass));
    dir *= -1;
    curdir[i + 1] = dir;
  }
  int q;
  cin >> q;
  REP(puella, 0, q) {
    ll t, a;
    cin >> t >> a;
    int idx = upper_bound(r.begin(), r.end(), t) - r.begin() - 1;
    ll diff = t - r[idx];
    ll base = max(el[idx], min(em[idx], a + en[idx]));
    ll ans = max(0LL, min(x, base + curdir[idx] * diff));
    cout << ans << endl;
  }
}
