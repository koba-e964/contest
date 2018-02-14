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
#include <unordered_map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

ll powmod(ll a, ll e) {
  ll sum = 1;
  ll cur = a;
  while (e > 0) {
    if (e % 2) {
      sum = sum * cur % mod;
    }
    cur = cur * cur % mod;
    e /= 2;
  }
  return sum;
}



void add(ll &x, ll y) {
  x = (x + y) % mod;
}
void mul(ll &x, ll y) {
  x = (x * y) % mod;
}

ll bern[7] = {1,(mod+1)/2,(mod+1)/6,0,powmod(mod-30,mod-2),0,powmod(42,mod-2)};
ll inv[8];
void init(){
  REP(i, 1, 8) inv[i] = powmod(i, mod - 2);
}
ll comb[8][8]={
  {1},
  {1,1},
  {1,2,1},
  {1,3,3,1},
  {1,4,6,4,1},
  {1,5,10,10,5,1},
  {1,6,15,21,15,6,1},
  {1,7,21,35,35,21,7,1},
};

unordered_map<ll, ll> memo[7];

ll accum(ll x, int d) {
  if (x == 0) {
    return d == 0 ? 1 : 0;
  }
  if (d == 0) {
    return (2 * x + 1) % mod;
  }
  /*
  if (memo[d].count(x)) {
    return memo[d][x];
  }
  */
  ll tot = 0;
  REP(i, 0, d + 1) {
    ll tmp = bern[i];
    REP(_, 0, d + 1 - i) {
      mul(tmp, x % mod);
    }
    mul(tmp, comb[d + 1][i]);
    add(tot,tmp);
  }
  mul(tot,inv[d+1]);
  mul(tot,2);
  return tot;
}

ll coef_final(ll m, int d) {
  ll lim = sqrt(m) + 2;
  ll tot = 0;
  ll ylim = lim;
  for(ll x = 0; x <= lim; ++x) {
    // cerr << "m="<<m<<" d="<<d<<" x="<<x<<" tot="<<tot<<endl;
    if (x * x > m) continue;
    while (ylim * ylim > m - x * x) {
      ylim--;
    }
    ll xx = x * x % mod;
    REP(i, 0, d + 1) {
      ll tmp = accum(ylim, 2 * i);
      REP(j, 0, d - i) mul(tmp, xx);
      mul(tmp, comb[d][i]);
      add(tot, tmp * (x == 0 ? 1 : 2));
    }
  }
  return tot;
}

ll smart2(ll m) {
  ll tot = 0;
  ll prod = m % mod;
  mul(prod, (m + 1) % mod);
  mul(prod, (m + 2) % mod);
  add(tot, coef_final(m, 0) * prod);
  add(tot, coef_final(m, 1) * ((3 * m + 4) % mod));
  prod = (3 * m + 6) % mod;
  mul(prod, (mod - 1) % mod);
  add(tot, coef_final(m, 2) * prod);
  add(tot, coef_final(m, 3) * 2);
  return tot * ((mod + 1) / 6) % mod;
}

int main(void) {
  init();
  ios::sync_with_stdio(false);
  cin.tie(0);
  ll m;
  cin >> m;
  cout << smart2(m) << "\n";
}
